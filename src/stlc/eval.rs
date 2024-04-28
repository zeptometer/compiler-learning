use crate::stlc::ast::Ast;
use crate::stlc::env;
use crate::stlc::env::Env;
use crate::stlc::val;
use crate::stlc::val::Val;
use std::rc::Rc;

pub fn eval(ast: Rc<Ast>, env: Rc<Env<Val>>, cont: Box<dyn FnOnce(Rc<Val>) -> Rc<Val>>) -> Rc<Val> {
    match &*ast {
        Ast::Int(i) => cont(Rc::new(Val::Int(*i))),
        Ast::Var(idx) => env::lookup(env, *idx)
            .map(|v| cont(v))
            .unwrap_or(Rc::new(Val::Error)),
        Ast::Lam(body) => cont(Rc::new(Val::Clos(env.clone(), body.clone()))),
        Ast::App(func, arg) => {
            let env2 = env.clone();
            let arg2 = arg.clone();
            eval(
                func.clone(),
                env,
                Box::new(|funcv| match &*funcv {
                    Val::Clos(cenv, body) => {
                        let body2 = body.clone();
                        let cenv2 = cenv.clone();
                        eval(
                            arg2,
                            env2,
                            Box::new(|argval| eval(body2, env::cons(argval, cenv2), cont)),
                        )
                    }
                    _ => val::error(),
                }),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stlc::ast;
    use crate::stlc::env::{cons, empty, Env};
    use crate::stlc::val;

    #[test]
    fn eval_literal() {
        fn env1() -> Rc<Env<val::Val>> {
            cons(val::int(10), cons(val::int(20), empty()))
        }
        assert_eq!(eval(ast::int(1), env1(), Box::new(|v| v)), val::int(1));
        assert_eq!(eval(ast::var(0), env1(), Box::new(|v| v)), val::int(10));
        assert_eq!(eval(ast::var(1), env1(), Box::new(|v| v)), val::int(20));
        assert_eq!(
            eval(
                ast::app(
                    ast::app(ast::lam(ast::lam(ast::var(1))), ast::int(33)),
                    ast::int(44)
                ),
                env1(),
                Box::new(|v| v)
            ),
            val::int(33)
        )
    }
}
