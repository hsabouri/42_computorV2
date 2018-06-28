use std::collections::HashMap;
use std::f32;
use ast::*;

pub struct Solver {
    vars: HashMap<String, Expr>,
    funcs: HashMap<String, (String, Expr)>
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            vars: HashMap::<String, Expr>::new(),
            funcs: HashMap::<String, (String, Expr)>::new(),
        }
    }

    fn reduce_matrice(&self, matrice: Vec<Vec<Box<Expr>>>) -> Result<Expr, String>{
        let mut res = Vec::<Vec<Box<Expr>>>::new();
        let mut errors = String::new();
        let mut x_size: Option<usize> = None;

        for (y, line) in matrice.iter().enumerate() {
            let mut res_line = Vec::<Box<Expr>>::new();

            x_size = match x_size {
                Some(a) if a != line.len() =>
                    return Err(format!("invalid dimensions\n\texpected: [{}, {}]\n\tfound: [{}, {}]",
                                       a, matrice.len(), line.len(), matrice.len())),
                None => Some(line.len()),
                a => a,
            };

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

    fn solve_function(&self, expr: Expr, arg_name: String, arg_value: Expr) -> Result<Expr, String> {
        Err(format!("Implement the god damn function solver"))
    }

    fn handle_complex(&self, a: f32, b: f32) -> Result<Expr, String> {
        if b < 0.0 + f32::EPSILON && b > 0.0 - f32::EPSILON {
            Ok(Expr::Number(a))
        } else {
            Ok(Expr::Complex(a, b))
        }
    }

    fn handle_variable(&self, s: String) -> Result<Expr, String> {
        match self.vars.get(&s.to_lowercase()).cloned() {
            Some(value) => Ok(value),
            None => Err(format!("Error: Variable '{}' is undefined.", s)),
        }
    }

    fn handle_function(&self, s: String, expr: Expr) -> Result<Expr, String> {
        match self.funcs.get(&s.to_lowercase()).cloned() {
            Some((arg_name, func_expr)) => {
                self.solve_function(func_expr, arg_name, expr)
            },
            None => Err(format!("Error: Function '{}' is undefined.", s))
        }
    }

    fn solve(&self, expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Number(a) => Ok(Expr::Number(a)),
            Expr::Imaginary => Ok(Expr::Imaginary),
            Expr::Complex(a, b) => self.handle_complex(a, b),
            Expr::Matrix(matrice) => self.reduce_matrice(matrice),
            Expr::Variable(s) => self.handle_variable(s),
            Expr::Function(s, expr) => self.handle_function(s, *expr),
            Expr::Op(a, op, b) => match op {
                Opcode::Add => self.solve(*a)? + self.solve(*b)?,
                Opcode::Mul => self.solve(*a)? * self.solve(*b)?,
                Opcode::Sub => self.solve(*a)? - self.solve(*b)?,
                Opcode::Div => self.solve(*a)? / self.solve(*b)?,
                Opcode::Rem => self.solve(*a)? % self.solve(*b)?,
                Opcode::Pow => self.solve(*a)?.pow(self.solve(*b)?),
                Opcode::Prod => self.solve(*a)?.prod(self.solve(*b)?),
                Opcode::ProdDiv => self.solve(*a)?.prod_div(self.solve(*b)?),
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
