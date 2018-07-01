use std::f32;
use std::fmt;

pub trait Pow<RHS=Self> {
    type Output;

    fn pow(self, rhs: RHS) -> Self::Output;
}

pub trait Prod<RHS=Self> {
    type Output;

    fn prod(self, rhs: RHS) -> Self::Output;
}

pub trait ProdDiv<RHS=Self> {
    type Output;

    fn prod_div(self, rhs: RHS) -> Self::Output;
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    Prod,
    ProdDiv,
}

#[derive(Debug)]
pub enum Input {
    Assignation(Box<Expr>, Box<Expr>),
    Eval(Box<Expr>)
}

#[derive(Clone)]
pub enum Expr {
    Number(f32),
    Imaginary,
    Complex(f32, f32),
    Matrix(Vec<Vec<Box<Expr>>>),
    Variable(String),
    Function(String, Box<Expr>),
    Op(Box<Expr>, Opcode, Box<Expr>)
}

impl Expr {
    pub fn type_error(left: Expr, right: Expr, op: Opcode) -> String {
        format!("Can't {:?} {} with {}", op,
            match left {
                Expr::Number(_) => format!("number ({})", left),
                Expr::Imaginary => format!("complex ({})", left),
                Expr::Complex(_, _) => format!("complex ({})", left),
                Expr::Matrix(_) => format!("matrice ({:?})", left),
                Expr::Variable(_) => format!("variable ({})", left),
                Expr::Function(_, _) => format!("function ({})", left),
                Expr::Op(_, _, _) => format!("expression ({})", left),
            },
            match right {
                Expr::Number(_) => format!("number ({})", right),
                Expr::Imaginary => format!("complex ({})", right),
                Expr::Complex(_, _) => format!("complex ({})", right),
                Expr::Matrix(_) => format!("matrice ({:?})", right),
                Expr::Variable(_) => format!("variable ({})", right),
                Expr::Function(_, _) => format!("function ({})", right),
                Expr::Op(_, _, _) => format!("expression ({})", right),
            }
        )
    }
}
