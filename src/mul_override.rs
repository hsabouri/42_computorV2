use ast::{Expr, Opcode};
use std::ops::Mul;

fn mul_number_complex(n: f32, c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(n * c.0, n * c.1))
}

fn mul_complex_complex(c0: (f32, f32), c1: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c0.0 * c1.0 - c0.1 * c1.1, c0.1 * c1.0 + c0.0 * c1.1))
}

fn mul_number_imaginary(n: f32) -> Result<Expr, String> {
    Ok(Expr::Complex(0.0, n))
}

fn mul_complex_imaginary(c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(-c.1, c.0))
}

fn mul_matrix_any(a: Vec<Vec<Box<Expr>>>, b: Expr) -> Result<Expr, String> {
    let mut res = Vec::<Vec<Box<Expr>>>::new();
    let mut errors = String::new();

    for (y, line) in a.iter().enumerate() {
        let mut res_line = Vec::<Box<Expr>>::new();

        for (x, value) in line.iter().enumerate() {
            let left = *value.clone();
            let right = b.clone();
            res_line.push(match left * right {
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

fn mul_matrix_matrix(a: Vec<Vec<Box<Expr>>>, b: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
    let mut res = Vec::<Vec<Box<Expr>>>::new();
    let mut errors = String::new();

    for (y, line) in a.iter().zip(b.iter()).enumerate() {
        let mut res_line = Vec::<Box<Expr>>::new();

        for (x, value) in line.0.iter().zip(line.1.iter()).enumerate() {
            let (left, right) = (*value.0.clone(), *value.1.clone());
            res_line.push(match left * right {
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

impl Mul for Expr {
    type Output = Result<Expr, String>;

    fn mul(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a * b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) | (Expr::Complex(ca, cb), Expr::Number(a)) =>
                mul_number_complex(a, (ca, cb)),
            (Expr::Complex(a, b), Expr::Complex(x, y)) => mul_complex_complex((a, b), (x, y)),
            (Expr::Number(a), Expr::Imaginary) | (Expr::Imaginary, Expr::Number(a)) =>
                mul_number_imaginary(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(ca, cb)) =>
                mul_complex_imaginary((ca, cb)),
            (Expr::Matrix(a), Expr::Number(b)) | (Expr::Number(b), Expr::Matrix(a)) =>
                mul_matrix_any(a, Expr::Number(b)),
            (Expr::Matrix(a), Expr::Complex(x, y)) | (Expr::Complex(x, y), Expr::Matrix(a)) =>
                mul_matrix_any(a, Expr::Complex(x, y)),
            (Expr::Matrix(a), Expr::Imaginary) | (Expr::Imaginary, Expr::Matrix(a)) =>
                mul_matrix_any(a, Expr::Imaginary),
            (Expr::Matrix(a), Expr::Matrix(b)) => mul_matrix_matrix(a, b), //Kronecker product
            (a, b) => Ok(Expr::Op(Box::new(a), Opcode::Mul, Box::new(b))),
        }
    }
}
