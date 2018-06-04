#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mult,
    Div,
    Rem,
    Pow,
}

#[derive(Debug)]
pub enum Input {
    Assignation(Box<Expr>, Box<Expr>),
    Eval(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(f32),
    Imaginary,
    Matrix(Vec<Vec<Expr>>),
    Variable(String),
    Function(String, Box<Expr>),
    Op(Box<Expr>, Opcode, Box<Expr>)
}
