use std::f32;

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

pub enum AbstractType {
    Computable(Expr),
    Litteral(Expr),
    Matrix(Expr),
}

impl AbstractType {
    pub fn get_expr(self) -> Expr {
        match self {
            AbstractType::Computable(e) |
            AbstractType::Litteral(e) |
            AbstractType::Matrix(e) => e
        }
    }
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

    pub fn get_abstract_type(self) -> AbstractType {
        match self {
            Expr::Number(a) => AbstractType::Computable(Expr::Number(a)),
            Expr::Complex(a, b) => AbstractType::Computable(Expr::Complex(a, b)),
            Expr::Imaginary => AbstractType::Computable(Expr::Imaginary),
            Expr::Variable(s) => AbstractType::Litteral(Expr::Variable(s)),
            Expr::Function(s, a) => AbstractType::Litteral(Expr::Function(s, a)),
            Expr::Matrix(v) => AbstractType::Matrix(Expr::Matrix(v)),
            a => AbstractType::Litteral(a)
        }
    }
}
