use crate::stlc::eval_defunct2::data::*;
use std::rc::Rc;

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::Cont0 => v,
        Cont::EvalArg(arg, env, cont) => match &*v {
            Val::Clos(cenv, cbody) => {
                (apply_compt(arg, env, Cont::EvalClos(cbody.clone(), cenv.clone(), cont))).clone()
            }
            _ => val::error(),
        },
        Cont::EvalClos(cbody, cenv, cont) => apply_compt(cbody, env::cons(v, cenv), *cont),
    }
}

fn apply_compt(compt: Rc<Compt>, env: Rc<Env>, cont: Cont) -> Rc<Val> {
    match &*compt {
        Compt::Lit(i) => apply_cont(cont, val::int(*i)),
        Compt::Access(idx) => env::lookup(env, *idx)
            .map(|v| apply_cont(cont, v))
            .unwrap_or(val::error()),
        Compt::Close(body) => apply_cont(cont, val::clos(env.clone(), body.clone())),
        Compt::Push(func, arg) => {
            apply_compt(func.clone(), env.clone(), Cont::EvalArg(arg.clone(), env, Box::new(cont)))
        }
    }
}

pub fn eval(ast: Rc<Ast>) -> Rc<Compt> {
    match &*ast {
        Ast::Int(i) => Rc::new(Compt::Lit(*i)),
        Ast::Var(idx) => Rc::new(Compt::Access(*idx)),
        Ast::Lam(body) => Rc::new(Compt::Close(eval(body.clone()))),
        Ast::App(func, arg) => {
            Rc::new(Compt::Push(eval(func.clone()), eval(arg.clone())))
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
        assert_eq!(
            apply_compt(eval(ast::int(1)), env1(), Cont::Cont0),
            val::int(1)
        );
        assert_eq!(
            apply_compt(eval(ast::var(0)), env1(), Cont::Cont0),
            val::int(10)
        );
        assert_eq!(
            apply_compt(eval(ast::var(1)), env1(), Cont::Cont0),
            val::int(20)
        );
        assert_eq!(
            apply_compt(
                eval(ast::app(
                    ast::app(ast::lam(ast::lam(ast::var(1))), ast::int(33)),
                    ast::int(44)
                )),
                env1(),
                Cont::Cont0
            ),
            val::int(33)
        )
    }
}
