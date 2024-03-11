use crate::ast::Ast;

pub fn eval(e: &Ast) -> i32 {
    match e {
        Ast::Int(i) => *i,
        Ast::Add(e1, e2) => eval(&*e1) + eval(&*e2),
        Ast::Mul(e1, e2) => eval(&*e1) * eval(&*e2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{int, add, mul};

    #[test]
    fn eval_literal() {
        assert_eq!(eval(&*int(1)), 1);
        assert_eq!(eval(&*int(10)), 10);
        assert_eq!(eval(&*int(-999)), -999);
    }

    #[test]
    fn eval_add() {
        assert_eq!(eval(&*add(int(1), int(2))), 3);
        assert_eq!(eval(&*add(int(0), add(int(2), int(3)))), 5);
        assert_eq!(eval(&*add(add(int(2), int(3)), int(-10))), -5);
    }

    #[test]
    fn eval_mul() {
        assert_eq!(eval(&*mul(int(1), int(2))), 2);
        assert_eq!(eval(&*mul(int(2), add(int(2), int(3)))), 10);
        assert_eq!(eval(&*mul(mul(int(2), int(3)), int(0))), 0);
    }
}
