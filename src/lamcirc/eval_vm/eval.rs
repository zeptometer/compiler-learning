use crate::lamcirc::eval_vm::data::*;
use std::rc::Rc;

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::End => v,
        Cont::EvalArg(arg, env, rest) => match &*v {
            Val::Clos(cenv, compbody) => {
                let compbody2 = compbody.clone();
                let cenv2 = cenv.clone();
                run(
                    arg,
                    0,
                    env,
                    Box::new(Cont::ReduceFunc(*compbody2, cenv2, rest)),
                )
            }
            _ => val::error("Expected closure"),
        },
        Cont::ReduceFunc(compbody, cenv, rest) => run(compbody, 0, env::cons(v, cenv), rest),
        Cont::ToQuo(rest) => match &*v {
            Val::Fut(normcode) => apply_cont(*rest, val::quo(normcode.clone())),
            _ => val::error("Expected future code fragment"),
        },
        Cont::RedQuo(rest) => match &*v {
            Val::Quo(normcode) => apply_cont(*rest, val::fut(normcode.clone())),
            _ => val::error("Expected quoted code"),
        },
        Cont::FutLam(rest) => match &*v {
            Val::Fut(normcode) => apply_cont(*rest, val::fut(ast::lam(normcode.clone()))),
            _ => val::error("Expected future code fratgment"),
        },
        Cont::FutAppArg(lev, comparg, env, cont) => match &*v {
            Val::Fut(funcnorm) => {
                let funcnorm2 = funcnorm.clone();
                run(comparg, lev, env, Box::new(Cont::FutApp(funcnorm2, cont)))
            }
            _ => val::error("Expected future code fratgment"),
        },
        Cont::FutApp(funcnorm, rest) => match &*v {
            Val::Fut(argnorm) => apply_cont(*rest, val::fut(ast::app(funcnorm, argnorm.clone()))),
            _ => val::error("Expected future code fratgment"),
        },
        Cont::FutQuo(rest) => match &*v {
            Val::Fut(codev) => apply_cont(*rest, val::fut(ast::quo(codev.clone()))),
            _ => val::error("Expected future code fragment"),
        },
        Cont::FutUnq(rest) => match &*v {
            Val::Fut(codev) => apply_cont(*rest, val::fut(ast::unq(codev.clone()))),
            _ => val::error("Expected future code fragment"),
        },
    }
}

pub fn run(compt: Compt, lev: i32, env: Rc<Env>, cont: Box<Cont>) -> Rc<Val> {
    return compt.0(lev, env, cont);
}

pub fn compile(ast: Rc<Ast>) -> Compt {
    //    println!("evaluating {:?} at level {}", &ast, lev);
    match &*ast {
        // Top-level evaluation
        Ast::Int(i) => Compt(Box::new(|lev, env, cont| match lev {
            0 => apply_cont(*cont, val::int(*i)),
            _ => apply_cont(*cont, val::fut(ast::int(*i))),
        })),
        Ast::Var(idx) => Compt(Box::new(|lev, env, cont| match lev {
            0 => env::lookup(env, *idx)
                .map(|v| apply_cont(*cont, v))
                .unwrap_or(val::error("Undefined variable")),
            _ => apply_cont(*cont, val::fut(ast::var(*idx))),
        })),
        Ast::Lam(body) => {
            let compbody = compile(body.clone());
            Compt(Box::new(|lev, env, cont| match lev {
                0 => apply_cont(*cont, val::clos(env.clone(), compbody)),
                lev => run(compbody, lev, env, Box::new(Cont::FutLam(cont))),
            }))
        }
        Ast::App(func, arg) => {
            let compfunc = compile(func.clone());
            let comparg = compile(arg.clone());
            Compt(Box::new(|lev, env, cont| match lev {
                0 => run(
                    compfunc,
                    0,
                    env.clone(),
                    Box::new(Cont::EvalArg(comparg, env, cont)),
                ),
                lev => run(
                    compfunc,
                    lev,
                    env.clone(),
                    Box::new(Cont::FutAppArg(lev, comparg, env, cont)),
                ),
            }))
        }
        Ast::Quo(code) => {
            let compcode = compile(code.clone());
            Compt(Box::new(|lev, env, cont| match lev {
                0 => run(compcode, 1, env, Box::new(Cont::ToQuo(cont))),
                lev => run(compcode, lev + 1, env, Box::new(Cont::FutQuo(cont))),
            }))
        }
        Ast::Unq(code) => {
            let compcode = compile(code.clone());
            Compt(Box::new(|lev, env, cont| match lev {
                0 => val::error("Top-level unquote is NOT allowed"),
                1 => run(compcode, 0, env, Box::new(Cont::RedQuo(cont))),
                lev => run(compcode, lev - 1, env, Box::new(Cont::FutUnq(cont))),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lamcirc::eval_vm::data::ast::*;
    use crate::lamcirc::eval_vm::data::env;
    use crate::lamcirc::eval_vm::data::val;

    fn env1() -> Rc<Env> {
        env::cons(val::int(10), env::cons(val::int(20), env::empty()))
    }

    #[test]
    fn eval_misc() {
        assert_eq!(
            run(compile(int(1)), 0, env1(), Box::new(Cont::End)),
            val::int(1)
        );
        assert_eq!(
            run(compile(var(0)), 0, env1(), Box::new(Cont::End)),
            val::int(10)
        );
        assert_eq!(
            run(compile(var(1)), 0, env1(), Box::new(Cont::End)),
            val::int(20)
        );
        assert_eq!(
            run(
                compile(app(app(lam(lam(var(1))), int(33)), int(44))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::int(33)
        )
    }

    #[test]
    fn eval_staged() {
        assert_eq!(
            run(compile(int(1)), 1, env1(), Box::new(Cont::End)),
            val::fut(int(1))
        );
        assert_eq!(
            run(compile(quo(int(1))), 0, env1(), Box::new(Cont::End)),
            val::quo(int(1))
        );
        assert_eq!(
            run(
                compile(quo(unq(quo(int(1))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(int(1))
        );
        assert_eq!(
            run(
                compile(quo(unq(quo(int(1))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(ast::int(1))
        );
        assert_eq!(
            run(
                compile(app(lam(quo(unq(var(0)))), quo(int(1)))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(int(1))
        );
        assert_eq!(
            run(
                compile(quo(lam(unq(quo(var(0)))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(lam(var(0)))
        );
        assert_eq!(
            run(
                compile(quo(app(var(0), unq(quo(int(1)))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(app(var(0), int(1)))
        );
    }

    #[test]
    fn eval_nested_quotes() {
        assert_eq!(
            run(
                compile(quo(quo(unq(var(0))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(quo(unq(var(0))))
        );
        assert_eq!(
            run(
                compile(quo(quo(unq(unq(quo(var(1))))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(quo(unq(var(1))))
        );
    }
}
