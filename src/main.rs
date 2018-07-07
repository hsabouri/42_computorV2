#![feature(box_patterns)]

extern crate rustyline;
extern crate snowflake;

mod parser;
mod solver;
mod ast;
mod add_override;
mod add_reduce;
mod sub_override;
mod sub_reduce;
mod mul_override;
mod mul_reduce;
mod div_override;
mod rem_override;
mod cmp_override;
mod fmt_override;
mod pow_trait;
mod prod_trait;
mod proddiv_trait;

use solver::Solver;
use ast::{Input, Expr};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn parse(solver: &mut Solver, line: String) {
    match parser::EquExprParser::new().parse(&line) {
        Ok(Input::Assignation(left, right)) => match solver.assign(*left, *right) {
            Ok(Expr::Function(name, arg)) => solver.show_function(Expr::Function(name, arg)),
            Ok(expr) => println!("{}", expr),
            Err(err) => println!("{}", err),
        },
        Ok(Input::Eval(expr)) => match solver.eval(*expr) {
            Ok(expr) => println!("{}", expr),
            Err(err) => println!("{}", err),
        },
        Err(err) => println!("{}", err),
    };
}

fn main() {
    let mut reader = Editor::<()>::new();
    let mut solver = Solver::new();

    match reader.load_history("computor_history") {
        Ok(_) => println!("history loaded"),
        Err(_) => println!("no history found")
    };
    loop {
        match reader.readline("computor > ") {
            Ok(line) => {
                if line.clone() == format!("exit") {break;}
                reader.add_history_entry(line.as_ref());
                parse(&mut solver, line);
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
