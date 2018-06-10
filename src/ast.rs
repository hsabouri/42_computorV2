use std::f32;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::fmt;

#[derive(Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Add => write!(f, "add"),
            Opcode::Sub => write!(f, "substract"),
            Opcode::Mul => write!(f, "multiply"),
            Opcode::Div => write!(f, "divide"),
            Opcode::Rem => write!(f, "modulo"),
            Opcode::Pow => write!(f, "power"),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Rem => write!(f, "%"),
            Opcode::Pow => write!(f, "^"),
        }
    }
}

#[derive(Debug)]
pub enum Input {
    Assignation(Box<Expr>, Box<Expr>),
    Eval(Box<Expr>)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f32),
    Imaginary,
    Matrix(Vec<Vec<Expr>>),
    Variable(String),
    Function(String, Box<Expr>),
    Op(Box<Expr>, Opcode, Box<Expr>)
}

impl Expr {
    fn operation_error(self, op: Opcode) -> String {
        match self {
            Expr::Number(_) => format!("Can't {:?} on numbers", op),
            Expr::Imaginary => format!("Can't {:?} on complex numbers", op),
            Expr::Matrix(_) => format!("Can't {:?} on matrices", op),
            Expr::Variable(_) => format!("Can't {:?} on raw variables", op),
            Expr::Function(_, _) => format!("Can't {:?} on raw functions", op),
            Expr::Op(_, _, _) => format!("Can't {:?} on raw exprs", op),
        }
    }

    pub fn pow(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b) => Ok(Expr::Number(a.powf(b))),
                b => Err(b.operation_error(Opcode::Add))
            },
            a => Err(a.operation_error(Opcode::Add))
        }
    }
}

impl Add for Expr {
    type Output = Result<Expr, String>;

    fn add(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b) => Ok(Expr::Number(a + b)),
                b => Err(b.operation_error(Opcode::Add))
            },
            a => Err(a.operation_error(Opcode::Add))
        }
    }
}

impl Sub for Expr {
    type Output = Result<Expr, String>;

    fn sub(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b) => Ok(Expr::Number(a - b)),
                b => Err(b.operation_error(Opcode::Sub))
            },
            a => Err(a.operation_error(Opcode::Sub))
        }
    }
}

impl Mul for Expr {
    type Output = Result<Expr, String>;

    fn mul(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b) => Ok(Expr::Number(a * b)),
                b => Err(b.operation_error(Opcode::Mul))
            },
            a => Err(a.operation_error(Opcode::Mul))
        }
    }
}

impl Div for Expr {
    type Output = Result<Expr, String>;

    fn div(self, other: Expr) -> Result<Expr, String> {
        match self {
            Expr::Number(a) => match other {
                Expr::Number(b)
                    if b >= 0.0 + f32::EPSILON &&
                    b <= 0.0 - f32::EPSILON => Ok(Expr::Number(a / b)),
                b => Err(b.operation_error(Opcode::Div))
            },
            a => Err(a.operation_error(Opcode::Div))
        }
    }
}

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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(ref a) => write!(f, "{}", a),
            Expr::Imaginary => write!(f, "i"),
            Expr::Matrix(_) => write!(f, "[...]"),
            Expr::Variable(ref s) => write!(f, "{}", s),
            Expr::Function(ref s, ref e) => write!(f, "{}({})", s, *e),
            Expr::Op(ref a, ref o, ref b) => write!(f, "{} {} {}", a, o, b)
        }
    }
}
