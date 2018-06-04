mod parser;
mod ast;

use ast::*;

fn main() {
    let expr = parser::EquExprParser::new()
        .parse("func(x) = 3 * x + 1 - kjsd ^ 18 * 2");
    println!("{:#?}", expr);
}
