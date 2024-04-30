use crate::lamcirc::eval_cps::data::*;
use std::rc::Rc;

pub fn eval(
    lev: i32,
    ast: Rc<Ast>,
    env: Rc<Env>,
    cont: Box<dyn FnOnce(Rc<Val>) -> Rc<Val>>,
) -> Rc<Val> {
    //    println!("evaluating {:?} at level {}", &ast, lev);
    match (lev, &*ast) {
        // Top-level evaluation
        (0, Ast::Int(i)) => cont(val::int(*i)),
        (0, Ast::Var(idx)) => env::lookup(env, *idx)
            .map(|v| cont(v))
            .unwrap_or(val::error("Undefined variable")),
        (0, Ast::Lam(body)) => cont(Rc::new(Val::Clos(env.clone(), body.clone()))),
        (0, Ast::App(func, arg)) => {
            let env2 = env.clone();
            let arg2 = arg.clone();
            eval(
                0,
                func.clone(),
                env,
                Box::new(|funcv| match &*funcv {
                    Val::Clos(cenv, body) => {
                        let body2 = body.clone();
                        let cenv2 = cenv.clone();
                        eval(
                            0,
                            arg2,
                            env2,
                            Box::new(|argval| eval(0, body2, env::cons(argval, cenv2), cont)),
                        )
                    }
                    _ => val::error("Expected closure"),
                }),
            )
        }
        (0, Ast::Quo(code)) => eval(
            1,
            code.clone(),
            env,
            Box::new(|codev| match &*codev {
                Val::Fut(normcode) => cont(val::quo(normcode.clone())),
                _ => val::error("Expected future code fragment"),
            }),
        ),
        (0, Ast::Unq(_)) => val::error("Top-level unquote is NOT allowed"),
        (1, Ast::Unq(code)) => eval(
            0,
            code.clone(),
            env,
            Box::new(|codev| match &*codev {
                Val::Quo(normcode) => cont(val::fut(normcode.clone())),
                _ => val::error("Expected quoted code"),
            }),
        ),
        // Future-level evaluation
        (_, Ast::Int(i)) => cont(val::fut(ast::int(*i))),
        (_, Ast::Var(idx)) => cont(val::fut(ast::var(*idx))),
        (lev, Ast::Lam(body)) => eval(
            lev,
            body.clone(),
            env,
            Box::new(|codev| match &*codev {
                Val::Fut(normcode) => cont(val::fut(ast::lam(normcode.clone()))),
                _ => val::error("Expected future code fratgment"),
            }),
        ),
        (lev, Ast::App(func, arg)) => {
            let arg2 = arg.clone();
            eval(
                lev,
                func.clone(),
                env.clone(),
                Box::new(move |funcv| match &*funcv {
                    Val::Fut(funcnorm) => {
                        let funcnorm2 = funcnorm.clone();
                        eval(
                            lev,
                            arg2,
                            env,
                            Box::new(|argv| match &*argv {
                                Val::Fut(argnorm) => {
                                    cont(val::fut(ast::app(funcnorm2, argnorm.clone())))
                                }
                                _ => val::error("Expected future code fratgment"),
                            }),
                        )
                    }
                    _ => val::error("Expected future code fratgment"),
                }),
            )
        }
        (_, Ast::Quo(code)) => cont(val::fut(ast::quo(code.clone()))),
        (_, Ast::Unq(code)) => cont(val::fut(ast::unq(code.clone()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lamcirc::eval_cps::data::ast::*;
    use crate::lamcirc::eval_cps::data::env;
    use crate::lamcirc::eval_cps::data::val;

    fn env1() -> Rc<Env> {
        env::cons(val::int(10), env::cons(val::int(20), env::empty()))
    }

    #[test]
    fn eval_misc() {
        assert_eq!(eval(0, int(1), env1(), Box::new(|v| v)), val::int(1));
        assert_eq!(eval(0, var(0), env1(), Box::new(|v| v)), val::int(10));
        assert_eq!(eval(0, var(1), env1(), Box::new(|v| v)), val::int(20));
        assert_eq!(
            eval(
                0,
                app(app(lam(lam(var(1))), int(33)), int(44)),
                env1(),
                Box::new(|v| v)
            ),
            val::int(33)
        )
    }

    #[test]
    fn eval_staged() {
        assert_eq!(eval(1, int(1), env1(), Box::new(|v| v)), val::fut(int(1)));
        assert_eq!(
            eval(0, quo(int(1)), env1(), Box::new(|v| v)),
            val::quo(int(1))
        );
        assert_eq!(
            eval(0, quo(unq(quo(int(1)))), env1(), Box::new(|v| v)),
            val::quo(int(1))
        );
        assert_eq!(
            eval(0, quo(unq(quo(int(1)))), env1(), Box::new(|v| v)),
            val::quo(ast::int(1))
        );
        assert_eq!(
            eval(
                0,
                app(lam(quo(unq(var(0)))), quo(int(1))),
                env1(),
                Box::new(|v| v)
            ),
            val::quo(int(1))
        );
        assert_eq!(
            eval(0, quo(lam(unq(quo(var(0))))), env1(), Box::new(|v| v)),
            val::quo(lam(var(0)))
        );
        assert_eq!(
            eval(
                0,
                quo(app(var(0), unq(quo(int(1))))),
                env1(),
                Box::new(|v| v)
            ),
            val::quo(app(var(0), int(1)))
        );
    }
}
