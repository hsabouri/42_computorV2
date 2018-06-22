use ast::{Prod, ProdDiv, Expr, Opcode};
use std::f32;

/*
fn matrice_invert(mat: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
    let invert = match (mat.len(), mat[0].len()) {
        (a, b) if a != b =>
            return Err(format!("non-square matrice [{}, {}]", a, b)),
        (2, 2) => match ((*mat[0][0] * *mat[1][1])? - (*mat[0][1] * *mat[1][0])?)? { 
            det if det >= 0.0 - f32::EPSILON && det >= 0.0 - f32::EPSILON =>
                return Err(format!("det({}) == 0", Expr::Matrix(mat))),
            det => (Expr::Number(1.0) / det)? * Expr::Matrix(mat) // Inverting 2x2 matrices
        },
        (a, b) => Err(format!("matrice of order [{}, {}]", a, b))
    };
    match invert {
        Ok(a) => Ok(a),
        Err(s) => Err(format!("can't invert: {}", s))
    }
}    
*/

impl ProdDiv for Expr {
    type Output = Result<Expr, String>;

    fn prod_div(self, other: Expr) -> Result<Expr, String> {
        Err(format!("Implement the damn productDivision trait"))
    }
}
