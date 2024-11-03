use crate::{
    functions::make_type_error,
    types::{BuiltinFunction, Cons, Function, Quote, Symbol},
};
use anyhow::Result;
use std::{fmt, rc::Rc};

pub enum Object {
    Integer(i32),
    Symbol(Symbol),
    Function(Function),
    BuiltinFunction(BuiltinFunction),
    Quote(Quote),
    Cons(Cons),
    Bool(bool),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(inner) => inner.fmt(f),
            Self::Symbol(inner) => inner.fmt(f),
            Self::Function(inner) => inner.fmt(f),
            Self::BuiltinFunction(inner) => inner.fmt(f),
            Self::Quote(inner) => inner.fmt(f),
            Self::Cons(inner) => inner.fmt(f),
            Self::Bool(inner) => inner.fmt(f),
        }
    }
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

    pub fn car(self: Rc<Self>) -> Result<Rc<Self>> {
        match &*self {
            Self::Cons(cons) => match &cons.0 {
                Some(_) => Ok(cons.car()),
                None => Ok(self), // We already have a nil, so let's reuse it
            },
            _ => Err(make_type_error("Object::car", &[&*self])),
        }
    }

    pub fn cdr(self: Rc<Self>) -> Result<Rc<Self>> {
        match &*self {
            Self::Cons(cons) => match &cons.0 {
                Some(_) => Ok(cons.cdr()),
                None => Ok(self), // We already have a nil, so let's reuse it
            },
            _ => Err(make_type_error("Object::cdr", &[&*self])),
        }
    }

    pub fn apply(&self, args: &Cons, env: &Cons) -> Result<(Rc<Self>, Cons)> {
        match self {
            Self::Function(func) => func.apply(args, env),
            Self::BuiltinFunction(func) => func.apply(args, env),
            _ => Err(make_type_error("apply_obj", &[self])),
        }
    }

    pub fn eval(self: Rc<Self>, env: &Cons) -> Result<(Rc<Self>, Cons)> {
        match &*self {
            Self::Integer(_) | Self::Bool(_) | Self::Function(_) | Self::BuiltinFunction(_) => {
                Ok((self, env.clone()))
            }
            Self::Cons(cons) => match &cons.0 {
                None => Ok((self, env.clone())),
                Some(_) => cons.eval(env),
            },
            Self::Symbol(symbol) => symbol.eval(env),
            Self::Quote(quote) => Ok((quote.0.clone(), env.clone())),
        }
    }
}
