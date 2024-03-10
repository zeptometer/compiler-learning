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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
