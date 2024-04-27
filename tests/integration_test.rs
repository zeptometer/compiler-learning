use compiler_learning::arith::ast::{int, add, mul};
use compiler_learning::arith::eval::eval;

#[test]
fn eval_random_things() {
    assert_eq!(eval(&*add(int(2), int(3))), 5);
    assert_eq!(eval(&*mul(int(2), int(3))), 6);
    assert_eq!(eval(&*mul(add(int(10), int(2)), int(3))), 36);
}
