use crate::stlc::ast::*;
use crate::stlc::env::*;
use crate::stlc::val::*;
use std::rc::Rc;

pub fn eval(ast: Rc<Ast>, env: Rc<Env<Val>>, cont: Box<dyn FnOnce(Rc<Val>) -> Rc<Val>>) -> Rc<Val> {
    match &*ast {
        Ast::Int(i) => cont(Rc::new(Val::Int(*i))),
        Ast::Var(idx) => lookup(env, *idx)
            .map(|v| cont(v))
            .unwrap_or(Rc::new(Val::Error)),
        Ast::Lam(body) => cont(Rc::new(Val::Clos(Rc::clone(&env), Rc::clone(body)))),
        Ast::App(func, arg) => {
            let env2 = Rc::clone(&env);
            let arg2 = Rc::clone(&arg);
            eval(
                Rc::clone(&func),
                env,
                Box::new(|funcv| match &*funcv {
                    Val::Clos(cenv, body) => {
                        let body2 = Rc::clone(body);
                        let cenv2 = Rc::clone(cenv);
                        eval(
                            arg2,
                            env2,
                            Box::new(|argval| eval(body2, cons(argval, cenv2), cont)),
                        )
                    }
                    _ => Rc::new(Val::Error),
                }),
            )
        }
    }
}
