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

    fn reduce_matrice(&self, matrice: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
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

    fn reduce_matrice_in_function(&self, matrice: Vec<Vec<Box<Expr>>>, arg_name: String, arg_value: Expr) -> Result<Expr, String> {
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
                res_line.push(match self.solve_function(*value, arg_name.clone(), arg_value.clone()) {
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
            None => Ok(Expr::Variable(s)),
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

    fn solve_function(&self, expr: Expr, arg_name: String, arg_value: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Number(a) => Ok(Expr::Number(a)),
            Expr::Imaginary => Ok(Expr::Imaginary),
            Expr::Complex(a, b) => self.handle_complex(a, b),
            Expr::Matrix(matrice) => self.reduce_matrice_in_function(matrice, arg_name, arg_value),
            Expr::Variable(ref s) if s.to_lowercase() == arg_name => Ok(arg_value),
            Expr::Variable(s) => self.handle_variable(s),
            Expr::Function(s, expr) => self.handle_function(s, self.solve_function(*expr, arg_name, arg_value)?),
            Expr::Op(a, op, b) => match op {
                Opcode::Add => self.solve_function(*a, arg_name.clone(), arg_value.clone())? + self.solve_function(*b, arg_name, arg_value)?,
                Opcode::Mul => self.solve_function(*a, arg_name.clone(), arg_value.clone())? * self.solve_function(*b, arg_name, arg_value)?,
                Opcode::Sub => self.solve_function(*a, arg_name.clone(), arg_value.clone())? - self.solve_function(*b, arg_name, arg_value)?,
                Opcode::Div => self.solve_function(*a, arg_name.clone(), arg_value.clone())? / self.solve_function(*b, arg_name, arg_value)?,
                Opcode::Rem => self.solve_function(*a, arg_name.clone(), arg_value.clone())? % self.solve_function(*b, arg_name, arg_value)?,
                Opcode::Pow => self.solve_function(*a, arg_name.clone(), arg_value.clone())?.pow(self.solve_function(*b, arg_name, arg_value)?),
                Opcode::Prod => self.solve_function(*a, arg_name.clone(), arg_value.clone())?.prod(self.solve_function(*b, arg_name, arg_value)?),
                Opcode::ProdDiv => self.solve_function(*a, arg_name.clone(), arg_value.clone())?.prod_div(self.solve_function(*b, arg_name, arg_value)?),
            },
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

    fn clean_matrix(&self, fun_str: String, arg_str: String, matrice: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
        let mut res = Vec::<Vec<Box<Expr>>>::new();
        let mut errors = String::new();

        for (y, line) in matrice.iter().enumerate() {
            let mut res_line = Vec::<Box<Expr>>::new();

            for (x, value) in line.iter().enumerate() {
                let value = value.clone();
                res_line.push(match self.clean_function(fun_str.clone(), arg_str.clone(), *value) {
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

    fn clean_function(&self, fun_str: String, arg_str: String, expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Variable(ref a) if a.to_lowercase() == arg_str => Ok(Expr::Variable(a.to_lowercase())),
            Expr::Variable(a) => self.handle_variable(a),
            Expr::Op(a, op, b) =>
                Ok(Expr::Op(Box::new(self.clean_function(fun_str.clone(), arg_str.clone(), *a)?),
                            op,
                            Box::new(self.clean_function(fun_str, arg_str, *b)?))),
            Expr::Function(ref s, _) if s.to_lowercase() == fun_str => Err(format!("recursive function: {}", s)),
            Expr::Function(s, expr) => Ok(Expr::Function(s.to_lowercase(), expr)),
            Expr::Matrix(a) => self.clean_matrix(fun_str, arg_str, a),
            any => Ok(any),
        }
    }

    fn assign_function(&mut self, fun_str: String, arg_str: String, expr: Expr) -> Result<Expr, String> {
        let fun_str = fun_str.to_lowercase();
        let arg_str = arg_str.to_lowercase();
        let expr = self.clean_function(fun_str.clone(), arg_str.clone(), expr)?;
        
        self.funcs.insert(fun_str.clone(), (arg_str.clone(), expr));
        Ok(Expr::Function(fun_str, Box::new(Expr::Variable(arg_str))))
    }

    fn assign_variable(&mut self, var_str: String, expr: Expr) -> Result<Expr, String> {
        let var_str = var_str.to_lowercase();
        let res = self.solve(expr)?;

        self.vars.insert(var_str, res.clone());
        Ok(res)
    }

    pub fn assign(&mut self, left: Expr, expr: Expr) -> Result<Expr, String> {
        match left {
            Expr::Variable(var_str) => self.assign_variable(var_str, expr),
            Expr::Function(fun_str, box Expr::Variable(arg_str)) =>
                self.assign_function(fun_str, arg_str, expr),
            Expr::Function(fun_str, box arg) => 
                return Err(format!("assigning to {} with expression argument: '{}'", fun_str, arg)),
            _ => return Err(format!("Can't assign to this type")),
        }
    }

    pub fn eval(&self, expr: Expr) -> Result<Expr, String> {
        self.solve(expr)
    }

    pub fn show_function(&self, expr: Expr) {
        match expr {
            Expr::Function(ref name, _) => {
                match self.funcs.get(&name.to_lowercase()).cloned() {
                    Some((arg, expr)) => println!("{}({}) = {}", name, arg, expr),
                    None => println!("Error: function '{}' is undefined.", name),
                }
            },
            _ => println!("not a function"),
        }
    }
}

