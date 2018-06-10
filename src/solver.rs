use std::collections::HashMap;
use ast::{Expr, Opcode};

pub struct Solver {
    vars: HashMap<String, Expr>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            vars: HashMap::<String, Expr>::new(),
        }
    }

    fn solve(&self, expr: Expr) -> Result<Expr, String> {
        let cloned_expr = expr.clone();
        let res = match expr {
            Expr::Number(_) => Ok(expr),
            Expr::Imaginary => Ok(expr),
            Expr::Matrix(_) => Ok(expr), //TODO
            Expr::Variable(s) => match self.vars.get(&s.to_lowercase()).cloned() {
                Some(value) => Ok(value),
                None => return Err(format!("Error: Variable '{}' is undefined.", s)),
            },
            Expr::Function(_, expr) => Ok(*expr), // TODO,
            Expr::Op(a, op, b) => match op {
                Opcode::Add => self.solve(*a)? + self.solve(*b)?,
                Opcode::Mul => self.solve(*a)? * self.solve(*b)?,
                Opcode::Sub => self.solve(*a)? - self.solve(*b)?,
                Opcode::Div => self.solve(*a)? / self.solve(*b)?,
                Opcode::Rem => self.solve(*a)? % self.solve(*b)?,
                Opcode::Pow => self.solve(*a)?.pow(self.solve(*b)?),
            },
        };
        match cloned_expr {
            Expr::Number(_) => print!(""),
            _ => println!("{} => {}", cloned_expr, res.clone().unwrap())
        };
        res
    }

    pub fn assign(&mut self, var: Expr, expr: Expr) -> Result<Expr, String> {
        let res = self.solve(expr)?;
        let var_str = match var {
            Expr::Variable(var_str) => var_str.to_lowercase(),
            _ => return Err(format!("Can only assign to variables")),
        };

        self.vars.insert(var_str, res.clone());
        Ok(res)
    }

    pub fn eval(&self, expr: Expr) -> Result<Expr, String> {
        self.solve(expr)
    }
}
