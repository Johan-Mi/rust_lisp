use crate::types::{Error, Object};
use std::{fmt, rc::Rc};

#[derive(Clone)]
pub enum Cons {
    Some(Rc<Object>, Rc<Object>),
    Nil,
}

impl Cons {
    pub fn len(&self) -> usize {
        match self {
            Self::Nil => 0,
            Self::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.len() + 1,
                _ => 1,
            },
        }
    }

    pub fn car(&self) -> Rc<Object> {
        match self {
            Self::Some(first, _) => first.clone(),
            Self::Nil => Rc::new(Object::Cons(Self::Nil)),
        }
    }

    pub fn cdr(&self) -> Rc<Object> {
        match self {
            Self::Some(_, second) => second.clone(),
            Self::Nil => Rc::new(Object::Cons(Self::Nil)),
        }
    }

    pub fn is_proper_list(&self) -> bool {
        match self {
            Self::Nil => true,
            Self::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.is_proper_list(),
                _ => false,
            },
        }
    }

    pub fn eval(&self, env: &Self) -> Result<(Rc<Object>, Self), Error> {
        if let Object::Cons(args) = &*self.cdr() {
            let (func, env) = self.car().eval(env)?;
            func.apply(args, &env)
        } else {
            Err(Error::new(
                "cdr of argument passed to eval_cons must be a cons".into(),
            ))
        }
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fn to_cons_string(obj: &Object) -> String {
            match obj {
                Object::Cons(Cons::Nil) => String::new(),
                Object::Cons(Cons::Some(first, second)) => {
                    format!(" {}{}", first, to_cons_string(second))
                }
                _ => format!(" . {obj}"),
            }
        }

        match self {
            Self::Nil => write!(formatter, "()"),
            Self::Some(first, second) => {
                write!(formatter, "({first}{})", to_cons_string(second))
            }
        }
    }
}
