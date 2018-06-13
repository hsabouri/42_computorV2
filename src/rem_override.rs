use ast::{Expr, Opcode};
use std::f32;
use std::ops::Rem;

fn rem_number_complex(a: f32, c: (f32, f32)) -> Result<Expr, String> {
    let n: i32 = a as i32;
    let (a, b) : (i32, i32) = (c.0 as i32, c.1 as i32);

    match (n, (a, b)) {
        (n, (a, _)) if a != 0 => Ok(Expr::Number((n % a) as f32)),
        _ => Err(format!("Can't modulo by 0")),
    }
}

fn rem_complex_complex(c0: (f32, f32), c1: (f32, f32)) -> Result<Expr, String> {
    let (a, b): (i32, i32) = (c0.0 as i32, c0.1 as i32);
    let (x, y): (i32, i32) = (c1.0 as i32, c1.1 as i32);

    match ((a, b), (x, y)) {
        ((a, b), (x, y)) if (x != 0 || a == 0) && (y != 0 || b == 0) => 
            Ok(Expr::Complex((a % x) as f32, (b % y) as f32)),
        _ => Err(format!("Can't modulo by 0")),
    }
}

fn rem_complex_imaginary(c: (f32, f32)) -> Result<Expr, String> {
    let (a, b) : (i32, i32) = (c.0 as i32, c.1 as i32);

    match (a, b) {
        (0, _) => Ok(Expr::Complex(0.0, 0.0)),
        _ => Err(format!("Can't modulo by 0")),
    }
}

fn rem_imaginary_complex(c: (f32, f32)) -> Result<Expr, String> {
    let (a, b) : (i32, i32) = (c.0 as i32, c.1 as i32);

    match (a, b) {
        (_, b) if b != 0 => Ok(Expr::Complex(0.0, (1 % b) as f32)),
        _ => Err(format!("Can't modulo by 0")),
    } 
}

impl Rem for Expr {
    type Output = Result<Expr, String>;

    fn rem(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) if b as i32 != 0 => Ok(Expr::Number((a as i32 % b as i32) as f32)),
            (Expr::Number(a), Expr::Complex(ca, cb)) => rem_number_complex(a, (ca, cb)),
            (Expr::Complex(c1a, c1b), Expr::Complex(c2a, c2b)) => rem_complex_complex((c1a, c1b), (c2a, c2b)),
            (Expr::Complex(ca, cb), Expr::Imaginary) => rem_complex_imaginary((ca, cb)),
            (Expr::Imaginary, Expr::Complex(ca, cb)) => rem_imaginary_complex((ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Rem)),
        }
    }
}
