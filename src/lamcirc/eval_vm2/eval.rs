use crate::lamcirc::eval_vm2::data::*;
use std::rc::Rc;

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::End => v,
        Cont::EvalArg(arg, env, rest) => match &*v {
            Val::Clos(cenv, compbody) => run(
                arg,
                0,
                env,
                Box::new(Cont::ReduceFunc(compbody.clone(), cenv.clone(), rest)),
            ),
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
            Val::Fut(normcode) => apply_cont(
                *rest,
                val::fut(instrseq::singleton(instr::clos(normcode.clone()))),
            ),
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
            Val::Fut(argnorm) => apply_cont(
                *rest,
                val::fut(instrseq::seq(instr::push(funcnorm), argnorm.clone())),
            ),
            _ => val::error("Expected future code fratgment"),
        },
        Cont::FutQuo(rest) => match &*v {
            Val::Fut(codev) => {
                apply_cont(*rest, val::fut(instrseq::seq(instr::ent(), codev.clone())))
            }
            _ => val::error("Expected future code fragment"),
        },
        Cont::FutUnq(rest) => match &*v {
            Val::Fut(codev) => apply_cont(
                *rest,
                val::fut(instrseq::seq(instr::leave(), codev.clone())),
            ),
            _ => val::error("Expected future code fragment"),
        },
    }
}

pub fn run(instrs: Rc<InstrSeq>, lev: i32, env: Rc<Env>, cont: Box<Cont>) -> Rc<Val> {
    match &*instrs {
        InstrSeq::Seq(Instr::Lit(i), _) => match lev {
            0 => apply_cont(*cont, val::int(*i)),
            _ => apply_cont(*cont, val::fut(instrseq::singleton(instr::lit(*i)))),
        },
        InstrSeq::Seq(Instr::Var(idx), _) => match lev {
            0 => env::lookup(env, *idx)
                .map(|v| apply_cont(*cont, v))
                .unwrap_or(val::error("Undefined variable")),
            _ => apply_cont(*cont, val::fut(instrseq::singleton(instr::var(*idx)))),
        },
        InstrSeq::Seq(Instr::Clos(body), _) => match lev {
            0 => apply_cont(*cont, val::clos(env.clone(), body.clone())),
            lev => run(body.clone(), lev, env, Box::new(Cont::FutLam(cont))),
        },
        InstrSeq::Seq(Instr::Push(func), arg) => match lev {
            0 => run(
                func.clone(),
                0,
                env.clone(),
                Box::new(Cont::EvalArg(arg.clone(), env, cont)),
            ),
            lev => run(
                func.clone(),
                lev,
                env.clone(),
                Box::new(Cont::FutAppArg(lev, arg.clone(), env, cont)),
            ),
        },
        InstrSeq::Seq(Instr::Ent, code) => match lev {
            0 => run(code.clone(), 1, env, Box::new(Cont::ToQuo(cont))),
            lev => run(code.clone(), lev + 1, env, Box::new(Cont::FutQuo(cont))),
        },
        InstrSeq::Seq(Instr::Leave, code) => match lev {
            0 => val::error("Top-level unquote is NOT allowed"),
            1 => run(code.clone(), 0, env, Box::new(Cont::RedQuo(cont))),
            lev => run(code.clone(), lev - 1, env, Box::new(Cont::FutUnq(cont))),
        },
        InstrSeq::End => val::error("Reached end of instruction sequence, unexpectedly"),
    }
}

pub fn compile(ast: Rc<Ast>) -> Rc<InstrSeq> {
    match &*ast {
        Ast::Int(i) => instrseq::singleton(instr::lit(*i)),
        Ast::Var(idx) => instrseq::singleton(instr::var(*idx)),
        Ast::Lam(body) => instrseq::singleton(instr::clos(compile(body.clone()))),
        Ast::App(func, arg) => {
            instrseq::seq(instr::push(compile(func.clone())), compile(arg.clone()))
        }
        Ast::Quo(code) => instrseq::seq(instr::ent(), compile(code.clone())),
        Ast::Unq(code) => instrseq::seq(instr::leave(), compile(code.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lamcirc::eval_vm2::data::ast::*;
    use crate::lamcirc::eval_vm2::data::env;
    use crate::lamcirc::eval_vm2::data::val;

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
            val::fut(instrseq::singleton(instr::lit(1)))
        );
        assert_eq!(
            run(compile(quo(int(1))), 0, env1(), Box::new(Cont::End)),
            val::quo(instrseq::singleton(instr::lit(1)))
        );
        assert_eq!(
            run(
                compile(quo(unq(quo(int(1))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::singleton(instr::lit(1)))
        );
        assert_eq!(
            run(
                compile(quo(unq(quo(int(1))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::singleton(instr::lit(1)))
        );
        assert_eq!(
            run(
                compile(app(lam(quo(unq(var(0)))), quo(int(1)))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::singleton(instr::lit(1)))
        );
        assert_eq!(
            run(
                compile(quo(lam(unq(quo(var(0)))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::singleton(instr::clos(instrseq::singleton(
                instr::var(0)
            ))))
        );
        assert_eq!(
            run(
                compile(quo(app(var(0), unq(quo(int(1)))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::seq(
                instr::push(instrseq::singleton(instr::var(0))),
                instrseq::singleton(instr::lit(1))
            ))
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
            val::quo(instrseq::seq(
                instr::ent(),
                instrseq::seq(instr::leave(), instrseq::singleton(instr::var(0)))
            ))
        );
        assert_eq!(
            run(
                compile(quo(quo(unq(unq(quo(var(1))))))),
                0,
                env1(),
                Box::new(Cont::End)
            ),
            val::quo(instrseq::seq(
                instr::ent(),
                instrseq::seq(instr::leave(), instrseq::singleton(instr::var(1)))
            ))
        );
    }
}
