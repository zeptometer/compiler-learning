#[derive(Eq, PartialEq, Debug)]
pub enum Ast {
    Int(i32),
    Add(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>)
}

// Constructors
pub fn int(i: i32) -> Box<Ast> {
    Box::new(Ast::Int(i))
}

pub fn add(e1: Box<Ast>, e2: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::Add(e1, e2))
}

pub fn mul(e1: Box<Ast>, e2: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::Mul(e1, e2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_literal() {
        assert_eq!(*int(1), Ast::Int(1));
    }

    #[test]
    fn make_add() {
        assert_eq!(*add(int(1), int(2)), Ast::Add(int(1), int(2)));
    }

    #[test]
    fn make_mul() {
        assert_eq!(*mul(int(1), int(2)), Ast::Mul(int(1), int(2)));
    }
}
