use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum Ast {
    Int(i32),
    Var(usize),
    Lam(Rc<Ast>),
    App(Rc<Ast>, Rc<Ast>),
    Quo(Rc<Ast>),
    Unq(Rc<Ast>)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Val {
    Error(String),
    Int(i32),
    Clos(Rc<Env>, Rc<Ast>),
    Quo(Rc<Ast>), // value for quoted code
    Fut(Rc<Ast>)  // frozen term of future stages
}

#[derive(Eq, PartialEq, Debug)]
pub enum Env {
    Nil,
    Cons(Rc<Val>, Rc<Env>)
}

pub mod ast {
    use super::*;

    pub fn int(i: i32) -> Rc<Ast> {
        Rc::new(Ast::Int(i))
    }

    pub fn var(i: usize) -> Rc<Ast> {
        Rc::new(Ast::Var(i))
    }

    pub fn lam(ast: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::Lam(ast))
    }

    pub fn app(ast1: Rc<Ast>, ast2: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::App(ast1, ast2))
    }

    pub fn quo(ast: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::Quo(ast))
    }

    pub fn unq(ast: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::Unq(ast))
    }
}

pub mod val {
    use super::*;

    pub fn error(msg: &str) -> Rc<Val> {
        return Rc::new(Val::Error(String::from(msg)));
    }

    pub fn int(i: i32) -> Rc<Val> {
        return Rc::new(Val::Int(i));
    }

    pub fn clos(env: Rc<Env>, body: Rc<Ast>) -> Rc<Val> {
        return Rc::new(Val::Clos(env, body));
    }

    pub fn quo(ast: Rc<Ast>) -> Rc<Val> {
        return Rc::new(Val::Quo(ast));
    }

    pub fn fut(ast: Rc<Ast>) -> Rc<Val> {
        return Rc::new(Val::Fut(ast));
    }
}

pub mod env {
    use super::*;

    pub fn empty() -> Rc<Env> {
        return Rc::new(Env::Nil);
    }

    pub fn cons(elm: Rc<Val>, env: Rc<Env>) -> Rc<Env> {
        return Rc::new(Env::Cons(elm.clone(), env.clone()));
    }

    pub fn lookup(env: Rc<Env>, idx: usize) -> Option<Rc<Val>> {
        match &*env {
            Env::Nil => Option::None,
            Env::Cons(head, tail) => {
                if idx == 0 {
                    return Some(head.clone());
                } else {
                    return lookup(tail.clone(), idx - 1);
                }
            }
        }
    }
}
