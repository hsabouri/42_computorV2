use ast::{Expr, Opcode};
use std::ops::Div;
use std::f32;

fn div_number_complex(a: f32, c: (f32, f32)) -> Result<Expr, String> {
    let squared = c.0 * c.0 - c.1 * c.1;

    if squared >= 0.0 - f32::EPSILON && squared <= 0.0 + f32::EPSILON {
        Err(format!("Can't divide by 0"))
    } else {
        Ok(Expr::Complex((a * c.0) / squared, -(a * c.1) / squared))
    }
}

fn div_complex_number(n: f32, c: (f32, f32)) -> Result<Expr, String> {
    if n >= 0.0 - f32::EPSILON && n <= 0.0 + f32::EPSILON {
        Err(format!("Can't divide by 0"))
    } else {
        Ok(Expr::Complex(c.0 / n, c.1 / n))
    }
}

fn div_complex_complex(c0: (f32, f32), c1: (f32, f32)) -> Result<Expr, String> {
    let squared = c1.0 * c1.0 + c1.1 * c1.1;
    
    if squared >= 0.0 - f32::EPSILON && squared <= 0.0 + f32::EPSILON {
        Err(format!("Can't divide by 0"))
    } else {
        Ok(Expr::Complex(
            (c0.0 * c1.0 + c0.1 + c1.1) / squared,
            (c0.1 * c1.0 - c0.0 * c1.1) / squared
        ))
    }
}

fn div_number_imaginary(n: f32) -> Result<Expr, String> {
    Ok(Expr::Complex(0.0, -n))
}

fn div_imaginary_number(n: f32) -> Result<Expr, String> {
    if n >= 0.0 - f32::EPSILON && n <= 0.0 + f32::EPSILON {
        Err(format!("Can't divide by 0"))
    } else {
        Ok(Expr::Complex(0.0, 1.0 / n))
    }
}

fn div_complex_imaginary(c: (f32, f32)) -> Result<Expr, String> {
    
    Ok(Expr::Complex(c.1, - c.0))
}

fn div_imaginary_complex(c: (f32, f32)) -> Result<Expr, String> {
    let squared = c.0 * c.0 - c.1 * c.1;
    
    if squared >= 0.0 - f32::EPSILON && squared <= 0.0 + f32::EPSILON {
        Err(format!("Can't divide by 0"))
    } else {
        Ok(Expr::Complex(c.1 / squared, c.0 / squared))
    }
}

impl Div for Expr {
    type Output = Result<Expr, String>;

    fn div(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Number(a), Expr::Number(b)) => Ok(Expr::Number(a / b)),
            (Expr::Number(a), Expr::Complex(ca, cb)) => div_number_complex(a, (ca, cb)),
            (Expr::Complex(ca, cb), Expr::Number(a)) => div_complex_number(a, (ca, cb)),
            (Expr::Complex(c1a, c1b), Expr::Complex(c2a, c2b)) => div_complex_complex((c1a, c1b), (c2a, c2b)),
            (Expr::Number(a), Expr::Imaginary) => div_number_imaginary(a),
            (Expr::Imaginary, Expr::Number(a)) => div_imaginary_number(a),
            (Expr::Complex(ca, cb), Expr::Imaginary) => div_complex_imaginary((ca, cb)),
            (Expr::Imaginary, Expr::Complex(ca, cb)) => div_imaginary_complex((ca, cb)),
            (a, b) => Err(Expr::type_error(a, b, Opcode::Div)),
        }
    }
}
