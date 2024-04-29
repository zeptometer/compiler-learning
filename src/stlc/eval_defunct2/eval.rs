use crate::stlc::eval_defunct2::data::*;
use std::rc::Rc;

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::Cont0 => v,
        Cont::EvalArg(arg, env, cont) => match &*v {
            Val::Clos(cenv, cbody) => {
                (&arg.0(env, Cont::EvalClos(cbody.clone(), cenv.clone(), cont))).clone()
            }
            _ => val::error(),
        },
        Cont::EvalClos(cbody, cenv, cont) => cbody.0(env::cons(v, cenv), *cont),
    }
}

pub fn eval(ast: Rc<Ast>) -> Compt {
    match &*ast {
        Ast::Int(i) => Compt(Rc::new(|env, cont| apply_cont(cont, Rc::new(Val::Int(*i))))),
        Ast::Var(idx) => Compt(Rc::new(|env, cont| {
            env::lookup(env, *idx)
                .map(|v| apply_cont(cont, v))
                .unwrap_or(val::error())
        })),
        Ast::Lam(body) => {
            let body_compt = eval(body.clone());
            Compt(Rc::new(|env, cont| {
                apply_cont(cont, Rc::new(Val::Clos(env.clone(), body_compt)))
            }))
        }
        Ast::App(func, arg) => {
            let func_compt = eval(func.clone());
            let arg_compt = eval(arg.clone());
            Compt(Rc::new(|env, cont| {
                func_compt.0(env.clone(), Cont::EvalArg(arg_compt, env, Box::new(cont)))
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stlc::eval_defunct2::data::env::{cons, empty};
    use crate::stlc::eval_defunct2::data::val;
    use crate::stlc::eval_defunct2::data::Env;

    #[test]
    fn eval_literal() {
        fn env1() -> Rc<Env> {
            cons(val::int(10), cons(val::int(20), empty()))
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
}
