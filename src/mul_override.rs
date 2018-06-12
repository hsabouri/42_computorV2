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

impl Mul for Expr {
    type Output = Result<Expr, String>;

    fn mul(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a * b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) | (Expr::Complex(ca, cb), Expr::Number(a)) =>
                mul_number_complex(a, (ca, cb)),
            (Expr::Number(a), Expr::Imaginary) | (Expr::Imaginary, Expr::Number(a)) =>
                mul_number_imaginary(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(ca, cb)) =>
                mul_complex_imaginary((ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Add)),
        }
    }
}
