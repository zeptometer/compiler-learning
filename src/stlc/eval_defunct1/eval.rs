use crate::stlc::eval_defunct1::ast::*;
use crate::stlc::eval_defunct1::env;
use crate::stlc::eval_defunct1::env::Env;
use crate::stlc::eval_defunct1::val;
use crate::stlc::eval_defunct1::val::Val;
use std::rc::Rc;

pub enum Cont {
    Cont0,
    EvalArg(Rc<Ast>, Rc<Env<Val>>, Box<Cont>),
    EvalClos(Rc<Ast>, Rc<Env<Val>>, Box<Cont>),
}

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::Cont0 => v,
        Cont::EvalArg(arg, env, cont) => match &*v {
            Val::Clos(cenv, cbody) => {
                eval(arg, env, Cont::EvalClos(cbody.clone(), cenv.clone(), cont))
            }
            _ => val::error(),
        },
        Cont::EvalClos(cbody, cenv, cont) => eval(cbody, env::cons(v, cenv), *cont),
    }
}

pub fn eval(ast: Rc<Ast>, env: Rc<Env<Val>>, cont: Cont) -> Rc<Val> {
    match &*ast {
        Ast::Int(i) => apply_cont(cont, Rc::new(Val::Int(*i))),
        Ast::Var(idx) => env::lookup(env, *idx)
            .map(|v| apply_cont(cont, v))
            .unwrap_or(Rc::new(Val::Error)),
        Ast::Lam(body) => apply_cont(cont, Rc::new(Val::Clos(env.clone(), body.clone()))),
        Ast::App(func, arg) => eval(
            func.clone(),
            env.clone(),
            Cont::EvalArg(arg.clone(), env, Box::new(cont)),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stlc::eval_defunct1::ast;
    use crate::stlc::eval_defunct1::env::{cons, empty, Env};
    use crate::stlc::eval_defunct1::val;

    #[test]
    fn eval_literal() {
        fn env1() -> Rc<Env<val::Val>> {
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
