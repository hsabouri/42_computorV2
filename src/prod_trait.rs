use ast::{Prod, Expr, Opcode};

fn prod_matrice_matrice(a: Vec<Vec<Box<Expr>>>, b: Vec<Vec<Box<Expr>>>) -> Result<Expr, String> {
    let types: (usize, usize) = (a[0].len(), b.len());
    let (n, m, p) = match types {
        (m1, m2) if m1 == m2 => (a.len(), m1, b[0].len()),
        _ => return Err(Expr::type_error(Expr::Matrix(a), Expr::Matrix(b), Opcode::Prod)),
    };
    let mut res = Vec::<Vec<Box<Expr>>>::with_capacity(n);

    for i in 0..n {
        let mut line = Vec::<Box<Expr>>::with_capacity(p);

        for j in 0..p {
            let mut sum = Expr::Number(0.0);

            for k in 0..m {
                sum = (sum + (*a[i][k].clone() * *b[k][j].clone())?)?;
            }
            line.push(Box::new(sum));
        }
        res.push(line);
    }
    Ok(Expr::Matrix(res))
}

impl Prod for Expr {
    type Output = Result<Expr, String>;

    fn prod(self, other: Expr) -> Result<Expr, String> {
        match (self, other) {
            (Expr::Matrix(a), Expr::Matrix(b)) => prod_matrice_matrice(a, b),
            (a, b) => Ok(Expr::Op(Box::new(a), Opcode::Prod, Box::new(b))),
        }
    }
}
