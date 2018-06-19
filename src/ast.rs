use std::f32;
use std::fmt;

pub trait Pow<RHS=Self> {
    type Output;

    fn pow(self, rhs: RHS) -> Self::Output;
}

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
                Expr::Matrix(_) => format!("matrice ({})", left),
                Expr::Variable(_) => format!("variable ({})", left),
                Expr::Function(_, _) => format!("function ({})", left),
                Expr::Op(_, _, _) => format!("expression ({})", left),
            },
            match right {
                Expr::Number(_) => format!("number ({})", right),
                Expr::Imaginary => format!("complex ({})", right),
                Expr::Complex(_, _) => format!("complex ({})", right),
                Expr::Matrix(_) => format!("matrice ({})", right),
                Expr::Variable(_) => format!("variable ({})", right),
                Expr::Function(_, _) => format!("function ({})", right),
                Expr::Op(_, _, _) => format!("expression ({})", right),
            }
        )
    }

    pub fn operation_error(self, op: Opcode) -> String {
        match self {
            Expr::Number(_) => format!("Can't {:?} on numbers", op),
            Expr::Imaginary => format!("Can't {:?} on imaginary number", op),
            Expr::Complex(_, _) => format!("Can't {:?} on complex numbers", op),
            Expr::Matrix(_) => format!("Can't {:?} on matrices", op),
            Expr::Variable(_) => format!("Can't {:?} on raw variables", op),
            Expr::Function(_, _) => format!("Can't {:?} on raw functions", op),
            Expr::Op(_, _, _) => format!("Can't {:?} on raw exprs", op),
        }
    }
}
  
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(ref a) => write!(f, "{}", a),
            Expr::Complex(ref a, ref b) => {
                if *b < 0.0 - f32::EPSILON {
                    write!(f, "{} {}i", a, b)
                } else {
                    write!(f, "{} + {}i", a, b)
                }
            },
            Expr::Imaginary => write!(f, "i"),
            Expr::Matrix(x) => {
                write!(f, "[ ")?;
                for (index, y) in x.iter().enumerate() {
                    write!(f, "[")?;
                    for (zindex, z) in y.iter().enumerate() {
                        if zindex < y.len() - 1 {
                            write!(f, "{}, ", z)?;
                        } else {
                            write!(f, "{}", z)?;
                        }
                    }
                    if index < x.len() - 1 {
                        write!(f, "] ; ")?;
                    } else {
                        write!(f, "]")?;
                    }
                }
                write!(f, " ]")
            },
            Expr::Variable(ref s) => write!(f, "{}", s),
            Expr::Function(ref s, ref e) => write!(f, "{}({})", s, *e),
            Expr::Op(ref a, ref o, ref b) => write!(f, "{} {} {}", a, o, b)
        }
    }
}
