use ast::*;
use solver::Solver;

pub enum Pattern {
    Trinome(Box<Expr>, Box<Expr>, Box<Expr>),
    Linear(Box<Expr>, Box<Expr>),
    Polynome2id(Box<Expr>, Box<Expr>),
    Polynome3id(Box<Expr>, Box<Expr>),
}

impl Pattern {
    pub new(expr: Expr) -> Option<Pattern> {
    }
}
