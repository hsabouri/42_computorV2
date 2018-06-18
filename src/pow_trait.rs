use ast::{Pow, Expr, Opcode};
use std::f32;

fn pow_complex_number(n: f32, c: Expr) -> Result<Expr, String> {
    let iterator = 0..(n as i32);
    let sigma = Expr::Number(n - (n as i32) as f32);
    let mut res = Expr::Number(1.0);

    for _ in iterator {
        if n >= 0.0 {
            res = (res * c.clone())?;
        } else {
            res = (res / c.clone())?;
        }
    }
    // TODO: Correct this monster
    if n >= 0.0 {
        Ok((res * (c * sigma)?)?)
    } else {
        Ok((res / (c * sigma)?)?)
    }
}

impl Pow for Expr {
    type Output = Result<Expr, String>;

    fn pow(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a.powf(b))),
            (Expr::Complex(ca, cb), Expr::Number(a)) => pow_complex_number(a, Expr::Complex(ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Pow))
        }
    }
}
