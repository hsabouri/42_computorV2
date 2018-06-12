use ast::{Expr, Opcode};
use std::ops::Sub;

fn sub_number_complex(n: f32, c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(n - c.0, -c.1))
}

fn sub_complex_number(n: f32, c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c.0 - n, c.1))
}

fn sub_complex_complex(c0: (f32, f32), c1: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c0.0 - c1.0, c0.1 - c1.1))
}

fn sub_number_imaginary(n: f32) -> Result<Expr, String> {
    Ok(Expr::Complex(n, -1.0))
}

fn sub_imaginary_number(n: f32) -> Result<Expr, String> {
    Ok(Expr::Complex(-n, 1.0))
}

fn sub_complex_imaginary(c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c.0, c.1 - 1.0))
}

fn sub_imaginary_complex(c: (f32, f32)) -> Result<Expr, String> {
    Ok(Expr::Complex(c.0, 1.0 - c.1))
}

impl Sub for Expr {
    type Output = Result<Expr, String>;

    fn sub(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a - b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) => sub_number_complex(a, (ca, cb)),
            (Expr::Complex(ca, cb), Expr::Number(a)) => sub_complex_number(a, (ca, cb)),
            (Expr::Complex(c1a, c1b), Expr::Complex(c2a, c2b)) => sub_complex_complex((c1a, c1b), (c2a, c2b)),
            (Expr::Number(a), Expr::Imaginary) => sub_number_imaginary(a),
            (Expr::Imaginary, Expr::Number(a)) => sub_imaginary_number(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) => sub_complex_imaginary((ca, cb)),
            (Expr::Imaginary, Expr::Complex(ca, cb)) => sub_imaginary_complex((ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Sub)),
        }
    }
}
