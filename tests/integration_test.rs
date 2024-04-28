#[test]
fn eval_random_things() {
    use compiler_learning::arith::ast::{int, add, mul};
    use compiler_learning::arith::eval::eval;

    assert_eq!(eval(&*add(int(2), int(3))), 5);
    assert_eq!(eval(&*mul(int(2), int(3))), 6);
    assert_eq!(eval(&*mul(add(int(10), int(2)), int(3))), 36);
}

#[test]
fn eval_defunct1() {
    use std::rc::Rc;
    use compiler_learning::stlc::ast;
    use compiler_learning::stlc::val;
    use compiler_learning::stlc::env;
    use compiler_learning::stlc::eval_defunct1::{eval, Cont};

    fn env1() -> Rc<env::Env<val::Val>> {
        env::cons(val::int(10), env::cons(val::int(20), env::empty()))
    }
    assert_eq!(eval(ast::int(1), env1(), Cont::Cont0), val::int(1));
    assert_eq!(eval(ast::var(0), env1(), Cont::Cont0), val::int(10));
    assert_eq!(eval(ast::var(1), env1(), Cont::Cont0), val::int(20));
    assert_eq!(
        eval(
            ast::app(
                ast::app(ast::lam(ast::lam(ast::var(1))), ast::int(33)),
                ast::int(44)
            ),
            env1(),
            Cont::Cont0
        ),
        val::int(33)
    )
}
