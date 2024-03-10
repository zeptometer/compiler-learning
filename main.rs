#[derive(Debug)]
enum Ast {
    Int(i32),
    Add(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>)
}

fn int(i: i32) -> Box<Ast> {
    Box::new(Ast::Int(i))
}

fn add(e1: Box<Ast>, e2: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::Add(e1, e2))
}

fn mul(e1: Box<Ast>, e2: Box<Ast>) -> Box<Ast> {
    Box::new(Ast::Mul(e1, e2))
}

fn eval(e: &Ast) -> i32 {
    match e {
        Ast::Int(i) => *i,
        Ast::Add(e1, e2) => eval(&*e1) + eval(&*e2),
        Ast::Mul(e1, e2) => eval(&*e1) * eval(&*e2),
    }
}

fn main() {
    let expr = mul(int(10),add(int(1), int(5)));
    println!("l = {:?} evals to {}", expr, eval(&*expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_literal() {
        assert_eq!(eval(&*int(1)), 1);
        assert_eq!(eval(&*int(10)), 10);
        assert_eq!(eval(&*int(-999)), -999);
    }

    #[test]
    fn eval_add() {
        assert_eq!(eval(&*add(int(1), int(2))), 3);
        assert_eq!(eval(&*add(int(0), add(int(2), int(3)))), 5);
        assert_eq!(eval(&*add(add(int(2), int(3)), int(-10))), -5);
    }

    #[test]
    fn eval_mul() {
        assert_eq!(eval(&*mul(int(1), int(2))), 2);
        assert_eq!(eval(&*mul(int(2), add(int(2), int(3)))), 10);
        assert_eq!(eval(&*mul(mul(int(2), int(3)), int(0))), 0);
    }
}
