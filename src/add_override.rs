use ast::{Expr, Opcode};
use std::ops::Add;
use add_reduce::{add_reduce_simple};

fn add_number_complex(n: f32, c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(n + c.0, c.1))
}

fn add_complex_complex(c0: (f32, f32), c1: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c0.0 + c1.0, c0.1 + c1.1))
}

fn add_number_imaginary(n: f32) -> Result<Expr, String> {
    Ok(Expr::Complex(n, 1.0))
}

fn add_complex_imaginary(c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c.0, c.1 + 1.0))
}

fn add_matrix_matrix(a: Vec<Vec<Box<Expr>>>, b: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
    let mut res = Vec::<Vec<Box<Expr>>>::new();
    let mut errors = String::new();

    for (y, line) in a.iter().zip(b.iter()).enumerate() {
        let mut res_line = Vec::<Box<Expr>>::new();

        for (x, value) in line.0.iter().zip(line.1.iter()).enumerate() {
            let (left, right) = (*value.0.clone(), *value.1.clone());
            res_line.push(match left + right {
                Ok(a) => Box::new(a),
                Err(s) => {
                    errors = if errors.is_empty() {
                        format!("{} at [{}, {}]", s, x, y)
                    } else {
                        format!("{}\n{} at [{}, {}]", errors, s, x, y)
                    };
                    Box::new(Expr::Number(0.0))
                },
            });
        }
        res.push(res_line);
    }
    if errors.is_empty() {
        Ok(Expr::Matrix(res))
    } else {
        Err(errors)
    }
}

impl Add for Expr {
    type Output = Result<Expr, String>;

    fn add(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a + b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) | (Expr::Complex(ca, cb), Expr::Number(a)) =>
                add_number_complex(a, (ca, cb)),
            (Expr::Complex(a, b), Expr::Complex(x, y)) => add_complex_complex((a, b), (x, y)),
            (Expr::Number(a), Expr::Imaginary) | (Expr::Imaginary, Expr::Number(a)) =>
                add_number_imaginary(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(ca, cb)) =>
                add_complex_imaginary((ca, cb)),
            (Expr::Matrix(a), Expr::Matrix(b)) => add_matrix_matrix(a, b),
            (Expr::Op(xa, opa, ya), Expr::Op(xb, opb, yb)) => Err(format!("Can't reduce Op Op")),
            (Expr::Op(a, op, b), c) | (c, Expr::Op(a, op, b)) => add_reduce_simple(*a, *b, c, op),
            (a, b) => Ok(Expr::Op(Box::new(a), Opcode::Add, Box::new(b))),
        }
    }
}
