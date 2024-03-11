mod ast;
mod eval;

use crate::ast::{int, add, mul};
use crate::eval::eval;

fn main() {
    let expr = mul(int(10),add(int(1), int(5)));
    println!("l = {:?} evals to {}", expr, eval(&*expr))
}
