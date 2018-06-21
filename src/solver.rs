use std::collections::HashMap;
use std::f32;
use ast::*;

pub struct Solver {
    vars: HashMap<String, Expr>,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            vars: HashMap::<String, Expr>::new(),
        }
    }

    fn reduce_matrice(&self, matrice: Vec<Vec<Box<Expr>>>) -> Result<Expr, String>{
        let mut res = Vec::<Vec<Box<Expr>>>::new();
        let mut errors = String::new();

        for (y, line) in matrice.iter().enumerate() {
            let mut res_line = Vec::<Box<Expr>>::new();

            for (x, value) in line.iter().enumerate() {
                let value = value.clone();
                res_line.push(match self.solve(*value) {
                    Ok(a) => Box::new(a),
                    Err(s) => {
                        errors = if errors.is_empty() {
                            format!("{} at [{}, {}]", s, x, y)
                        } else {
                            format!("{}\n{} at [{}, {}]", errors, s, x, y)
                        };
                        Box::new(Expr::Number(0.0))
                    },
                });
            }
            res.push(res_line);
        }
        if errors.is_empty() {
            Ok(Expr::Matrix(res))
        } else {
            Err(errors)
        }
    }

    fn solve(&self, expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Number(_) => Ok(expr),
            Expr::Imaginary => Ok(expr),
            Expr::Complex(a, b) => {
                if b < 0.0 + f32::EPSILON && b > 0.0 - f32::EPSILON {
                    Ok(Expr::Number(a))
                } else {
                    Ok(expr)
                }
            },
            Expr::Matrix(matrice) => self.reduce_matrice(matrice),
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
                Opcode::Prod => self.solve(*a)?.prod(self.solve(*b)?),
            },
        }
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
