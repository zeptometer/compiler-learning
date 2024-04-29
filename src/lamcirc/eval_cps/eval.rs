use crate::lamcirc::eval_cps::data::*;
use std::rc::Rc;

pub fn eval(
    ast: Rc<Ast>,
    lev: i32,
    env: Rc<Env>,
    cont: Box<dyn FnOnce(Rc<Val>) -> Rc<Val>>,
) -> Rc<Val> {
    match (&*ast, lev) {
        // Top-level evaluation
        (Ast::Int(i), 0) => cont(val::int(*i)),
        (Ast::Var(idx), 0) => env::lookup(env, *idx)
            .map(|v| cont(v))
            .unwrap_or(val::error()),
        (Ast::Lam(body), 0) => cont(Rc::new(Val::Clos(env.clone(), body.clone()))),
        (Ast::App(func, arg), 0) => {
            let env2 = env.clone();
            let arg2 = arg.clone();
            eval(
                func.clone(),
                0,
                env,
                Box::new(|funcv| match &*funcv {
                    Val::Clos(cenv, body) => {
                        let body2 = body.clone();
                        let cenv2 = cenv.clone();
                        eval(
                            arg2,
                            0,
                            env2,
                            Box::new(|argval| eval(body2, 0, env::cons(argval, cenv2), cont)),
                        )
                    }
                    _ => val::error(),
                }),
            )
        }
        (Ast::Quo(code), 0) => eval(
            code.clone(),
            1,
            env,
            Box::new(|codev| match &*codev {
                Val::Fut(normcode) => cont(val::quo(normcode.clone())),
                _ => val::error(),
            }),
        ),
        (Ast::Unq(_), 0) => val::error(), // Top-level unquote is NOT allowed
        (Ast::Unq(code), 1) => eval(
            code.clone(),
            0,
            env,
            Box::new(|codev| match &*codev {
                Val::Quo(normcode) => cont(val::fut(normcode.clone())),
                _ => val::error(),
            }),
        ),
        // Future-level evaluation
        (Ast::Int(i), _) => cont(val::fut(ast::int(*i))),
        (Ast::Var(idx), _) => cont(val::fut(ast::var(*idx))),
        (Ast::Lam(body), _) => cont(val::fut(ast::lam(body.clone()))),
        (Ast::App(func, arg), _) => cont(val::fut(ast::app(func.clone(), arg.clone()))),
        (Ast::Quo(code), _) => cont(val::fut(ast::quo(code.clone()))),
        (Ast::Unq(code), _) => cont(val::fut(ast::unq(code.clone()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lamcirc::eval_cps::data::ast;
    use crate::lamcirc::eval_cps::data::env;
    use crate::lamcirc::eval_cps::data::val;

    #[test]
    fn eval_misc() {
        fn env1() -> Rc<Env> {
            env::cons(val::int(10), env::cons(val::int(20), env::empty()))
        }
        assert_eq!(eval(ast::int(1), 0, env1(), Box::new(|v| v)), val::int(1));
        assert_eq!(eval(ast::var(0), 0, env1(), Box::new(|v| v)), val::int(10));
        assert_eq!(eval(ast::var(1), 0, env1(), Box::new(|v| v)), val::int(20));
        assert_eq!(
            eval(
                ast::app(
                    ast::app(ast::lam(ast::lam(ast::var(1))), ast::int(33)),
                    ast::int(44)
                ),
                0,
                env1(),
                Box::new(|v| v)
            ),
            val::int(33)
        )
    }
}
