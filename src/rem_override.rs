use ast::{Expr, Opcode};
use std::f32;
use std::ops::Rem;

impl Rem for Expr {
    type Output = Result<Expr, String>;

    fn rem(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b) if b >= 0.0 + f32::EPSILON || b <= 0.0 - f32::EPSILON => {
                    let (a, b) = (a as i32, b as i32);

                    Ok(Expr::Number((a % b) as f32))
                },
                b => Err(b.operation_error(Opcode::Rem))
            },
            a => Err(a.operation_error(Opcode::Rem))
        }
    }
}
