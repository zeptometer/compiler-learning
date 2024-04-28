use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum Env<T> {
    Nil,
    Cons(Rc<T>, Rc<Env<T>>)
}

pub fn empty<T>() -> Rc<Env<T>> {
    return Rc::new(Env::Nil)
}

pub fn cons<T>(elm: Rc<T>, env: Rc<Env<T>>) -> Rc<Env<T>> {
    return Rc::new(Env::Cons(Rc::clone(&elm), Rc::clone(&env)))
}

pub fn lookup<T>(env: Rc<Env<T>>, idx: usize) -> Option<Rc<T>> {
    match &*env {
        Env::Nil => Option::None,
        Env::Cons(head, tail) =>
        if idx == 0 {
            return Some(Rc::clone(&head))
        } else {
            return lookup(Rc::clone(&tail), idx-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn make_nil() {
        assert_eq!(empty::<i32>(), Rc::new(Env::Nil));
    }
}
