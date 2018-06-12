use ast::{Expr, Opcode};
use std::ops::Add;

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

impl Add for Expr {
    type Output = Result<Expr, String>;

    fn add(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a + b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) | (Expr::Complex(ca, cb), Expr::Number(a)) =>
                add_number_complex(a, (ca, cb)),
            (Expr::Number(a), Expr::Imaginary) | (Expr::Imaginary, Expr::Number(a)) =>
                add_number_imaginary(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(ca, cb)) =>
                add_complex_imaginary((ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Add)),
        }
    }
}
