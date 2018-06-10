extern crate rustyline;

mod parser;
mod ast;

use std::collections::HashMap;
use ast::{Expr, Input, Opcode};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn solve(vars: &HashMap<String, Expr>, expr: Expr) -> Result<Expr, String> {
    let expr2 = expr.clone();
    let res = match expr {
        Expr::Number(_) => Ok(expr),
        Expr::Imaginary => Ok(expr),
        Expr::Matrix(_) => Ok(expr), //TODO
        Expr::Variable(s) => match vars.get(&s.to_lowercase()).cloned() {
            Some(value) => Ok(value),
            None => return Err(format!("Error: Variable '{}' is undefined.", s)),
        },
        Expr::Function(_, expr) => Ok(*expr), // TODO,
        Expr::Op(a, op, b) => match op {
            Opcode::Add => solve(vars, *a)? + solve(vars, *b)?,
            Opcode::Mul => solve(vars, *a)? * solve(vars, *b)?,
            Opcode::Sub => solve(vars, *a)? - solve(vars, *b)?,
            Opcode::Div => solve(vars, *a)? / solve(vars, *b)?,
            Opcode::Rem => solve(vars, *a)? % solve(vars, *b)?,
            Opcode::Pow => solve(vars, *a)?.pow(solve(vars, *b)?),
        },
    };
    match expr2 {
        Expr::Number(_) => print!(""),
        _ => println!("{} => {}", expr2, res.clone().unwrap())
    }
    res
}

fn assign(vars: &mut HashMap<String, Expr>, var: Expr, expr: Expr) -> Result<Expr, String> {
    let res = solve(&vars, expr)?;
    let var_str = match var {
        Expr::Variable(var_str) => var_str.to_lowercase(),
        _ => return Err(format!("Can only assign to variables")),
    };

    vars.insert(var_str, res.clone());
    Ok(res)
}

fn eval(vars: &HashMap<String, Expr>, expr: Expr) -> Result<Expr, String> {
   solve(&vars, expr)
}

fn parse(vars: &mut HashMap<String, Expr>, line: String) {
    match parser::EquExprParser::new().parse(&line) {
        Ok(Input::Assignation(left, right)) => match assign(vars, *left, *right) {
            Ok(expr) => println!("{}", expr),
            Err(err) => println!("{}", err)
        },
        Ok(Input::Eval(expr)) => match eval(&vars, *expr) {
            Ok(expr) => println!("{}", expr),
            Err(err) => println!("{}", err)
        },
        Err(err) => println!("{}", err),
    };
}

fn main() {
    let mut reader = Editor::<()>::new();
    let mut vars: HashMap<String, Expr> = HashMap::new();

    reader.load_history("computor_history");
    loop {
        match reader.readline("computor > ") {
            Ok(line) => {
                if line.clone() == format!("exit") {break;}
                reader.add_history_entry(line.as_ref());
                parse(&mut vars, line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    reader.save_history("computor_history").unwrap();
}
