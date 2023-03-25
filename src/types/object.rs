use crate::{
    functions::make_type_error,
    types::{BuiltinFunction, Cons, Error, Function, Quote, Symbol},
};
use derive_more::Display;
use std::rc::Rc;

#[derive(Display)]
pub enum Object {
    Integer(i32),
    Symbol(Symbol),
    Function(Function),
    BuiltinFunction(BuiltinFunction),
    Quote(Quote),
    Cons(Cons),
    Bool(bool),
}

impl Object {
    pub const fn name_of_contained(&self) -> &str {
        match self {
            Self::Integer(_) => "(type int)",
            Self::Symbol(_) => "(type symbol)",
            Self::Function(_) => "(type function)",
            Self::BuiltinFunction(_) => "(type builtin-function)",
            Self::Quote(_) => "(type quote)",
            Self::Cons(_) => "(type cons)",
            Self::Bool(_) => "(type bool)",
        }
    }

    pub fn car(self: Rc<Self>) -> Result<Rc<Self>, Error> {
        match &*self {
            Self::Cons(cons) => match cons {
                Cons::Some(..) => Ok(cons.car()),
                Cons::Nil => Ok(self), // We already have a nil,
                                       // so let's reuse it
            },
            _ => Err(make_type_error("Object::car", &[&*self])),
        }
    }

    pub fn cdr(self: Rc<Self>) -> Result<Rc<Self>, Error> {
        match &*self {
            Self::Cons(cons) => match cons {
                Cons::Some(..) => Ok(cons.cdr()),
                Cons::Nil => Ok(self), // We already have a nil,
                                       // so let's reuse it
            },
            _ => Err(make_type_error("Object::cdr", &[&*self])),
        }
    }

    pub fn apply(
        &self,
        args: &Cons,
        env: &Cons,
    ) -> Result<(Rc<Self>, Cons), Error> {
        match self {
            Self::Function(func) => func.apply(args, env),
            Self::BuiltinFunction(func) => func.apply(args, env),
            _ => Err(make_type_error("apply_obj", &[self])),
        }
    }

    pub fn eval(self: Rc<Self>, env: &Cons) -> Result<(Rc<Self>, Cons), Error> {
        match &*self {
            Self::Integer(_)
            | Self::Bool(_)
            | Self::Function(_)
            | Self::BuiltinFunction(_) => Ok((self, env.clone())),
            Self::Cons(cons) => match cons {
                Cons::Nil => Ok((self, env.clone())),
                _ => cons.eval(env),
            },
            Self::Symbol(symbol) => symbol.eval(env),
            Self::Quote(quote) => Ok(((*quote).clone(), env.clone())),
        }
    }
}
