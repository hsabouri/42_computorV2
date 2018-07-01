use ast::{Expr, Opcode};
use std::f32;
use std::fmt;

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Opcode::Add => write!(f, "add"),
            Opcode::Sub => write!(f, "substract"),
            Opcode::Mul => write!(f, "multiply"),
            Opcode::Div => write!(f, "divide"),
            Opcode::Rem => write!(f, "modulo"),
            Opcode::Pow => write!(f, "power"),
            Opcode::Prod => write!(f, "matrice product"),
            Opcode::ProdDiv => write!(f, "matrice divide"),
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
            Opcode::Prod => write!(f, "**"),
            Opcode::ProdDiv => write!(f, "//"),
        }
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Matrix(x) => write!(f, "[{}, {}]", x.len(), x[0].len()),
            n => write!(f, "{}", n),
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
