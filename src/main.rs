mod arith;
mod stlc;

use std::rc::Rc;

fn main() {
    {
        use crate::arith::ast::{int, add, mul};
        use crate::arith::eval::eval;

        let expr = mul(int(10),add(int(1), int(5)));
        println!("l = {:?} evals to {}", expr, eval(&*expr))
    }

    {
        use crate::stlc::eval_cps::ast::{int, var, lam, app};
        use crate::stlc::eval_cps::env::empty;
        use crate::stlc::eval_cps::eval::eval;

        let expr = app(lam(var(0)), int(20));
        println!("l = {:?}  evals to {:?}", Rc::clone(&expr), eval(expr, empty(), Box::new(|v| v)))
    }
}
