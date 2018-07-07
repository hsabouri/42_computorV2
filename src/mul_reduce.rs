use ast::{Expr, Opcode};

fn add_sub_reduce(a: Expr, b: Expr, c: Expr, op: Opcode) -> Result<Expr, String> {
    Ok(
        Expr::Op(
            Box::new((a * c.clone())?),
            op,
            Box::new((b * c)?),
        )
    )
}

pub fn mul_reduce_simple(a: Expr, b: Expr, c: Expr, op: Opcode) -> Result<Expr, String> {
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
                    Opcode::Add,
                    Box::new(c)
                )
            )
        }
    }
}
