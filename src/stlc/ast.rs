#[derive(Eq, PartialEq, Debug)]
pub enum Ast {
    Int(i32),
    Var(i32),
    Lam(Box<Ast>),
    App(Box<Ast>, Box<Ast>)
}

pub enum Val {
    Clos(Env, Ast)
}

pub type Env = Vec<Val>;

// Constructors
pub fn int(i: i32) -> Box<Ast> {
    Box::new(Ast::Int(i))
}

pub fn var(i: i32) -> Box<Ast> {
    Box::new(Ast::Var(i))
}

pub fn lam(a: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::Lam(a))
}

pub fn app(a1: Box<Ast>, a2: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::App(a1, a2))
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
