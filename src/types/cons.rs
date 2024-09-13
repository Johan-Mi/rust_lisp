use crate::types::Object;
use anyhow::{bail, Result};
use std::{fmt, rc::Rc};

#[derive(Clone)]
pub struct Cons(pub Option<(Rc<Object>, Rc<Object>)>);

impl Cons {
    pub fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some((_, next)) => match &**next {
                Object::Cons(rest) => rest.len() + 1,
                _ => 1,
            },
        }
    }

    pub fn car(&self) -> Rc<Object> {
        match &self.0 {
            Some((first, _)) => first.clone(),
            None => Rc::new(Object::Cons(Self(None))),
        }
    }

    pub fn cdr(&self) -> Rc<Object> {
        match &self.0 {
            Some((_, second)) => second.clone(),
            None => Rc::new(Object::Cons(Self(None))),
        }
    }

    pub fn is_proper_list(&self) -> bool {
        match &self.0 {
            None => true,
            Some((_, next)) => match &**next {
                Object::Cons(rest) => rest.is_proper_list(),
                _ => false,
            },
        }
    }

    pub fn eval(&self, env: &Self) -> Result<(Rc<Object>, Self)> {
        let Object::Cons(args) = &*self.cdr() else {
            bail!("cdr of argument passed to eval_cons must be a cons");
        };
        let (func, env) = self.car().eval(env)?;
        func.apply(args, &env)
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fn to_cons_string(obj: &Object) -> String {
            match obj {
                Object::Cons(Cons(None)) => String::new(),
                Object::Cons(Cons(Some((first, second)))) => {
                    format!(" {}{}", first, to_cons_string(second))
                }
                _ => format!(" . {obj}"),
            }
        }

        match &self.0 {
            None => write!(formatter, "()"),
            Some((first, second)) => {
                write!(formatter, "({first}{})", to_cons_string(second))
            }
        }
    }
}
