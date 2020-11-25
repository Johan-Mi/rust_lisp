use super::error::*;
use super::object::*;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Cons {
    Some(Rc<Object>, Rc<Object>),
    Nil,
}

impl Cons {
    pub fn len(&self) -> usize {
        match self {
            Cons::Nil => 0,
            Cons::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.len() + 1,
                _ => 1,
            },
        }
    }

    pub fn car(&self) -> Rc<Object> {
        match self {
            Cons::Some(first, _) => first.clone(),
            Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
        }
    }

    pub fn cdr(&self) -> Rc<Object> {
        match self {
            Cons::Some(_, second) => second.clone(),
            Cons::Nil => Rc::new(Object::Cons(Cons::Nil)),
        }
    }

    pub fn is_proper_list(&self) -> bool {
        match self {
            Cons::Nil => true,
            Cons::Some(_, next) => match &**next {
                Object::Cons(rest) => rest.is_proper_list(),
                _ => false,
            },
        }
    }

    pub fn eval(&self, env: &Cons) -> Result<(Rc<Object>, Cons), Error> {
        match &*self.cdr() {
            Object::Cons(args) => {
                let (func, env) = eval_obj(self.car(), env)?;
                apply_obj(&func, &args, &env)
            }
            _ => Err(Error::new(String::from(
                "cdr of argument passed to eval_cons must be a cons",
            ))),
        }
    }
}

impl fmt::Display for Cons {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fn to_cons_string(obj: &Object) -> String {
            match obj {
                Object::Cons(cons) => match cons {
                    Cons::Nil => String::new(),
                    Cons::Some(first, second) => {
                        format!(" {}{}", first, to_cons_string(second))
                    }
                },
                _ => format!(" . {}", obj),
            }
        }

        match self {
            Cons::Nil => write!(formatter, "()"),
            Cons::Some(first, second) => {
                write!(formatter, "({}{})", first, to_cons_string(second))
            }
        }
    }
}
