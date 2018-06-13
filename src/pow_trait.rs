use ast::{Pow, Expr, Opcode};

/*
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
*/

impl Pow for Expr {
    type Output = Result<Expr, String>;

    fn pow(self, other: Expr) -> Result<Expr, String> {
        Err(format!("Implement the power trait you dumb !"))
    }
}