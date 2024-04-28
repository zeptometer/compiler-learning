use std::rc::Rc;
use crate::stlc::eval_cps::ast::Ast;
use crate::stlc::eval_cps::env::Env;

#[derive(Eq, PartialEq, Debug)]

pub enum Val {
    Error,
    Int(i32),
    Clos(Rc<Env<Val>>, Rc<Ast>)
}

pub fn error() -> Rc<Val> {
    return Rc::new(Val::Error)
}

pub fn int(i: i32) -> Rc<Val> {
    return Rc::new(Val::Int(i))
}

pub fn clos(env: Rc<Env<Val>>, body: Rc<Ast>) -> Rc<Val> {
    return Rc::new(Val::Clos(env, body))
}
