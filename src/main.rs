mod arith;

use crate::arith::ast::{int, add, mul};
use crate::arith::eval::eval;

fn main() {
    let expr = mul(int(10),add(int(1), int(5)));
    println!("l = {:?} evals to {}", expr, eval(&*expr))
}
