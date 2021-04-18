use crate::{
    functions::make_type_error,
    types::{
        Bool, BuiltinFunction, Cons, Error, Function, Integer, Quote, Symbol,
    },
};
use derive_more::Display;
use std::rc::Rc;

#[derive(Display)]
pub enum Object {
    Integer(Integer),
    Symbol(Symbol),
    Function(Function),
    BuiltinFunction(BuiltinFunction),
    Quote(Quote),
    Cons(Cons),
    Bool(Bool),
}

impl Object {
    pub fn name_of_contained(&self) -> &str {
        match self {
            Object::Integer(_) => "(type int)",
            Object::Symbol(_) => "(type symbol)",
            Object::Function(_) => "(type function)",
            Object::BuiltinFunction(_) => "(type builtin-function)",
            Object::Quote(_) => "(type quote)",
            Object::Cons(_) => "(type cons)",
            Object::Bool(_) => "(type bool)",
        }
    }

    pub fn car(self: Rc<Self>) -> Result<Rc<Object>, Error> {
        match &*self {
            Object::Cons(cons) => match cons {
                Cons::Some(..) => Ok(cons.car()),
                Cons::Nil => Ok(self), // We already have a nil,
                                       // so let's reuse it
            },
            _ => Err(make_type_error("Object::car", &[&*self])),
        }
    }

    pub fn cdr(self: Rc<Self>) -> Result<Rc<Object>, Error> {
        match &*self {
            Object::Cons(cons) => match cons {
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
    ) -> Result<(Rc<Object>, Cons), Error> {
        match self {
            Object::Function(func) => func.apply(args, env),
            Object::BuiltinFunction(func) => func.apply(args, env),
            _ => Err(make_type_error("apply_obj", &[self])),
        }
    }

    pub fn eval(
        self: Rc<Self>,
        env: &Cons,
    ) -> Result<(Rc<Object>, Cons), Error> {
        match &*self {
            Object::Integer(_)
            | Object::Bool(_)
            | Object::Function(_)
            | Object::BuiltinFunction(_) => Ok((self, env.clone())),
            Object::Cons(cons) => match cons {
                Cons::Nil => Ok((self, env.clone())),
                _ => cons.eval(env),
            },
            Object::Symbol(symbol) => symbol.eval(env),
            Object::Quote(quote) => Ok(((*quote).clone(), env.clone())),
        }
    }
}
