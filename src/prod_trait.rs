use ast::{Prod, Expr, Opcode};
use std::f32;

impl Prod for Expr {
    type Output = Result<Expr, String>;

    fn prod(self, other: Expr) -> Result<Expr, String> {
        Err(format!("Implement the damn product trait"))
    }
}
