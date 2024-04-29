use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum Ast {
    Int(i32),
    Var(usize),
    Lam(Rc<Ast>),
    App(Rc<Ast>, Rc<Ast>),
}

#[derive(Eq, PartialEq, Debug)]

pub enum Val {
    Error,
    Int(i32),
    Clos(Rc<Env>, Compt),
}

pub struct Compt(pub Rc<dyn Fn(Rc<Env>, Cont) -> Rc<Val>>);

impl std::fmt::Debug for Compt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{# some computation #}")
    }
}

impl PartialEq for Compt {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Compt {
}

impl Clone for Compt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Env {
    Nil,
    Cons(Rc<Val>, Rc<Env>),
}

pub enum Cont {
    Cont0,
    EvalArg(Compt, Rc<Env>, Box<Cont>),
    EvalClos(Compt, Rc<Env>, Box<Cont>),
}

pub mod ast {
    use super::*;

    pub fn int(i: i32) -> Rc<Ast> {
        Rc::new(Ast::Int(i))
    }

    pub fn var(i: usize) -> Rc<Ast> {
        Rc::new(Ast::Var(i))
    }

    pub fn lam(a: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::Lam(Rc::clone(&a)))
    }

    pub fn app(a1: Rc<Ast>, a2: Rc<Ast>) -> Rc<Ast> {
        Rc::new(Ast::App(Rc::clone(&a1), Rc::clone(&a2)))
    }
}

pub mod val {
    use super::*;

    pub fn error() -> Rc<Val> {
        return Rc::new(Val::Error);
    }

    pub fn int(i: i32) -> Rc<Val> {
        return Rc::new(Val::Int(i));
    }

    pub fn clos(env: Rc<Env>, body: Compt) -> Rc<Val> {
        return Rc::new(Val::Clos(env, body));
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

#[cfg(test)]
mod tests {
    use super::*;

    mod ast_tests {
        use super::*;
        use super::ast::*;

        #[test]
        fn make_literal() {
            assert_eq!(*int(1), Ast::Int(1));
        }

        #[test]
        fn make_var() {
            assert_eq!(*var(1), Ast::Var(1));
        }

        #[test]
        fn make_lam() {
            assert_eq!(*lam(var(1)), Ast::Lam(var(1)));
        }

        #[test]
        fn make_app() {
            assert_eq!(*app(lam(var(0)), var(2)), Ast::App(lam(var(0)), var(2)));
        }
    }

    mod env_tests {
        use super::*;
        use super::env::*;

        #[test]
        fn make_nil() {
            assert_eq!(empty(), Rc::new(Env::Nil));
        }
    }
}
