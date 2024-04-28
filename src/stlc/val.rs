use std::rc::Rc;
use crate::stlc::ast::Ast;
use crate::stlc::env::Env;

#[derive(Eq, PartialEq, Debug)]

pub enum Val {
    Error,
    Int(i32),
    Clos(Rc<Env<Val>>, Rc<Ast>)
}
