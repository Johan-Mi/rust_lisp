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
