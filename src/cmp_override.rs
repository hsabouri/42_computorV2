use ast::{Expr, Opcode};
use std::cmp::{Ordering, Eq};
use std::f32;

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Expr) -> Option<Ordering> {
        match (self.clone(), other.clone()) {
            (Expr::Number(a), Expr::Number(b)) => a.partial_cmp(&b),
            (Expr::Number(a), Expr::Complex(x, y)) | (Expr::Complex(x, y), Expr::Number(a)) =>
                a.partial_cmp(&(x * x + y * y).sqrt()),
            (Expr::Complex(a, b), Expr::Complex(x, y)) =>
                (a * a + b * b).sqrt().partial_cmp(&(x * x + y * y).sqrt()),
            (Expr::Complex(a, b), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(a, b)) =>
                (1.0).partial_cmp(&(a * a + b * b).sqrt()),
            _ => None,
        }
    }
}

fn cmp_matrix_matrix(a: Vec<Vec<Box<Expr>>>, b: Vec<Vec<Box<Expr>>>) -> bool {
    if a.len() != b.len() || a[0].len() != b[0].len() {
        false
    } else {
        let mut state = true;
        for (line_a, line_b) in a.iter().zip(b.iter()) {
            for (value_a, value_b) in line_a.iter().zip(line_b.iter()) {
                if value_a != value_b {
                    state = false;
                    break;
                }
            }
        }
        state
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        match (self.clone(), other.clone()) {
            (Expr::Number(a), Expr::Number(b)) =>
                a >= b - f32::EPSILON && a <= b + f32::EPSILON,
            (Expr::Number(a), Expr::Complex(x, y)) | (Expr::Complex(x, y), Expr::Number(a)) =>
                y >= 0.0 - f32::EPSILON && y <= 0.0 + f32::EPSILON &&
                a >= x - f32::EPSILON && a <= x + f32::EPSILON,
            (Expr::Complex(a, b), Expr::Complex(x, y)) =>
                a >= x - f32::EPSILON && a <= x + f32::EPSILON &&
                b >= y - f32::EPSILON && b <= y + f32::EPSILON,
            (Expr::Complex(_, b), Expr::Imaginary) | (Expr::Imaginary, Expr::Complex(_, b)) =>
                b >= 1.0 - f32::EPSILON && b <= 1.0 + f32::EPSILON,
            (Expr::Matrix(a), Expr::Matrix(b)) => cmp_matrix_matrix(a, b),
            _ => false,
        }
    }
}

impl Eq for Expr {}
