use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum Ast {
    Int(i32),
    Var(usize),
    Lam(Rc<Ast>),
    App(Rc<Ast>, Rc<Ast>)
}

// Constructors
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

#[cfg(test)]
mod tests {
    use super::*;


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
