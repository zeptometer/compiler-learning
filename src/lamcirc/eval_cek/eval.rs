use crate::lamcirc::eval_cek::data::*;
use std::rc::Rc;

enum Cont {
    Cont0,
    Cont1(Rc<Ast>, Rc<Env>, Box<Cont>),
    Cont2(Rc<Ast>, Rc<Env>, Box<Cont>),
    Cont3(Box<Cont>),
    Cont4(Box<Cont>),
    Cont5(Box<Cont>),
    Cont6(i32, Rc<Ast>, Rc<Env>, Box<Cont>),
    Cont7(Rc<Ast>, Box<Cont>),
    Cont8(Box<Cont>),
    Cont9(Box<Cont>),
}

fn apply_cont(cont: Cont, v: Rc<Val>) -> Rc<Val> {
    match cont {
        Cont::Cont0 => v,
        Cont::Cont1(arg, env, rest) => match &*v {
            Val::Clos(cenv, body) => {
                let body2 = body.clone();
                let cenv2 = cenv.clone();
                eval(0, arg, env, Cont::Cont2(body2, cenv2, rest))
            }
            _ => val::error("Expected closure"),
        },
        Cont::Cont2(body, cenv, rest) => eval(0, body, env::cons(v, cenv), *rest),
        Cont::Cont3(rest) => match &*v {
            Val::Fut(normcode) => apply_cont(*rest, val::quo(normcode.clone())),
            _ => val::error("Expected future code fragment"),
        },
        Cont::Cont4(rest) => match &*v {
            Val::Quo(normcode) => apply_cont(*rest, val::fut(normcode.clone())),
            _ => val::error("Expected quoted code"),
        },
        Cont::Cont5(rest) => match &*v {
            Val::Fut(normcode) => apply_cont(*rest, val::fut(ast::lam(normcode.clone()))),
            _ => val::error("Expected future code fratgment"),
        },
        Cont::Cont6(lev, arg, env, cont) => match &*v {
            Val::Fut(funcnorm) => {
                let funcnorm2 = funcnorm.clone();
                eval(lev, arg, env, Cont::Cont7(funcnorm2, cont))
            }
            _ => val::error("Expected future code fratgment"),
        },
        Cont::Cont7(funcnorm, rest) => match &*v {
            Val::Fut(argnorm) => apply_cont(*rest, val::fut(ast::app(funcnorm, argnorm.clone()))),
            _ => val::error("Expected future code fratgment"),
        },
        Cont::Cont8(rest) => match &*v {
            Val::Fut(codev) => apply_cont(*rest, val::fut(ast::quo(codev.clone()))),
            _ => val::error("Expected future code fragment"),
        },
        Cont::Cont9(rest) => match &*v {
            Val::Fut(codev) => apply_cont(*rest, val::fut(ast::unq(codev.clone()))),
            _ => val::error("Expected future code fragment"),
        },
    }
}

pub fn eval(lev: i32, ast: Rc<Ast>, env: Rc<Env>, cont: Cont) -> Rc<Val> {
    //    println!("evaluating {:?} at level {}", &ast, lev);
    match (lev, &*ast) {
        // Top-level evaluation
        (0, Ast::Int(i)) => apply_cont(cont, val::int(*i)),
        (0, Ast::Var(idx)) => env::lookup(env, *idx)
            .map(|v| apply_cont(cont, v))
            .unwrap_or(val::error("Undefined variable")),
        (0, Ast::Lam(body)) => apply_cont(cont, Rc::new(Val::Clos(env.clone(), body.clone()))),
        (0, Ast::App(func, arg)) => eval(
            0,
            func.clone(),
            env.clone(),
            Cont::Cont1(arg.clone(), env, Box::new(cont)),
        ),
        (0, Ast::Quo(code)) => eval(1, code.clone(), env, Cont::Cont3(Box::new(cont))),
        (0, Ast::Unq(_)) => val::error("Top-level unquote is NOT allowed"),
        (1, Ast::Unq(code)) => eval(0, code.clone(), env, Cont::Cont4(Box::new(cont))),
        // Future-level evaluation
        (_, Ast::Int(i)) => apply_cont(cont, val::fut(ast::int(*i))),
        (_, Ast::Var(idx)) => apply_cont(cont, val::fut(ast::var(*idx))),
        (lev, Ast::Lam(body)) => eval(lev, body.clone(), env, Cont::Cont5(Box::new(cont))),
        (lev, Ast::App(func, arg)) => eval(
            lev,
            func.clone(),
            env.clone(),
            Cont::Cont6(lev, arg.clone(), env, Box::new(cont)),
        ),
        (lev, Ast::Quo(code)) => eval(lev + 1, code.clone(), env, Cont::Cont8(Box::new(cont))),
        (_, Ast::Unq(code)) => eval(lev - 1, code.clone(), env, Cont::Cont9(Box::new(cont))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lamcirc::eval_cek::data::ast::*;
    use crate::lamcirc::eval_cek::data::env;
    use crate::lamcirc::eval_cek::data::val;

    fn env1() -> Rc<Env> {
        env::cons(val::int(10), env::cons(val::int(20), env::empty()))
    }

    #[test]
    fn eval_misc() {
        assert_eq!(eval(0, int(1), env1(), Cont::Cont0), val::int(1));
        assert_eq!(eval(0, var(0), env1(), Cont::Cont0), val::int(10));
        assert_eq!(eval(0, var(1), env1(), Cont::Cont0), val::int(20));
        assert_eq!(
            eval(
                0,
                app(app(lam(lam(var(1))), int(33)), int(44)),
                env1(),
                Cont::Cont0
            ),
            val::int(33)
        )
    }

    #[test]
    fn eval_staged() {
        assert_eq!(eval(1, int(1), env1(), Cont::Cont0), val::fut(int(1)));
        assert_eq!(eval(0, quo(int(1)), env1(), Cont::Cont0), val::quo(int(1)));
        assert_eq!(
            eval(0, quo(unq(quo(int(1)))), env1(), Cont::Cont0),
            val::quo(int(1))
        );
        assert_eq!(
            eval(0, quo(unq(quo(int(1)))), env1(), Cont::Cont0),
            val::quo(ast::int(1))
        );
        assert_eq!(
            eval(
                0,
                app(lam(quo(unq(var(0)))), quo(int(1))),
                env1(),
                Cont::Cont0
            ),
            val::quo(int(1))
        );
        assert_eq!(
            eval(0, quo(lam(unq(quo(var(0))))), env1(), Cont::Cont0),
            val::quo(lam(var(0)))
        );
        assert_eq!(
            eval(0, quo(app(var(0), unq(quo(int(1))))), env1(), Cont::Cont0),
            val::quo(app(var(0), int(1)))
        );
    }

    #[test]
    fn eval_nested_quotes() {
        assert_eq!(
            eval(0, quo(quo(unq(var(0)))), env1(), Cont::Cont0),
            val::quo(quo(unq(var(0))))
        );
        assert_eq!(
            eval(0, quo(quo(unq(unq(quo(var(1)))))), env1(), Cont::Cont0),
            val::quo(quo(unq(var(1))))
        );
    }
}
