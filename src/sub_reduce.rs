use ast::{Expr, Opcode, AbstractType};

fn add_sub_reduce(a: Expr, b: Expr, c: Expr, op: Opcode) -> Result<Expr, String> {
    match (a.get_abstract_type(), b.get_abstract_type(), c.get_abstract_type()) {
        (AbstractType::Computable(a), AbstractType::Litteral(b), AbstractType::Computable(c)) | 
        (AbstractType::Litteral(b), AbstractType::Computable(a), AbstractType::Computable(c)) |
        (AbstractType::Computable(a), AbstractType::Matrix(b), AbstractType::Computable(c)) | 
        (AbstractType::Matrix(b), AbstractType::Computable(a), AbstractType::Computable(c)) |
        (AbstractType::Matrix(a), AbstractType::Computable(b), AbstractType::Matrix(c)) | 
        (AbstractType::Computable(b), AbstractType::Matrix(a), AbstractType::Matrix(c)) =>
            Ok(
                Expr::Op(
                    Box::new((a - c)?),
                    op,
                    Box::new(b)
                )
            ),
        (a, b, c) => Ok(
            Expr::Op(
                Box::new(Expr::Op(
                    Box::new(a.get_expr()),
                    op,
                    Box::new(b.get_expr()),
                )),
                Opcode::Add,
                Box::new(c.get_expr()),
            )
        )
    }
}

pub fn sub_reduce_simple(a: Expr, b: Expr, c: Expr, op: Opcode) -> Result<Expr, String> {
    match op {
        Opcode::Add => add_sub_reduce(a, b, c, Opcode::Add),
        Opcode::Sub => add_sub_reduce(a, b, c, Opcode::Sub),
        op => {
            Ok(
                Expr::Op(
                    Box::new(Expr::Op(
                        Box::new(a),
                        op,
                        Box::new(b)
                    )),
                    Opcode::Sub,
                    Box::new(c)
                )
            )
        }
    }
}
