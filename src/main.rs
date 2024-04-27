mod arith;
mod stlc;

fn main() {
    {
        use crate::arith::ast::{int, add, mul};
        use crate::arith::eval::eval;

        let expr = mul(int(10),add(int(1), int(5)));
        println!("l = {:?} evals to {}", expr, eval(&*expr))
    }

    {
        use crate::stlc::ast::{int, var, lam, app};

        let expr = app(lam(var(0)), int(20));
        println!("l = {:?}", expr)
    }

}
