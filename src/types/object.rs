use super::boolean::*;
use super::builtin_function::*;
use super::cons::*;
use super::error::*;
use super::function::*;
use super::integer::*;
use super::quote::*;
use super::symbol::*;
use crate::functions::*;
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
}

pub fn car_obj(obj: Rc<Object>) -> Result<Rc<Object>, Error> {
    match &*obj {
        Object::Cons(cons) => match cons {
            Cons::Some(..) => Ok(cons.car()),
            Cons::Nil => Ok(obj), // We already have a nil, so let's reuse it
        },
        _ => Err(make_type_error("car_obj", &[&*obj])),
    }
}

pub fn cdr_obj(obj: Rc<Object>) -> Result<Rc<Object>, Error> {
    match &*obj {
        Object::Cons(cons) => match cons {
            Cons::Some(..) => Ok(cons.cdr()),
            Cons::Nil => Ok(obj), // We already have a nil, so let's reuse it
        },
        _ => Err(make_type_error("cdr_obj", &[&*obj])),
    }
}

pub fn apply_obj(
    func_obj: &Object,
    args: &Cons,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match func_obj {
        Object::Function(func) => func.apply(args, env),
        Object::BuiltinFunction(func) => func.apply(args, env),
        _ => Err(make_type_error("apply_obj", &[&*func_obj])),
    }
}

pub fn eval_obj(
    obj: Rc<Object>,
    env: &Cons,
) -> Result<(Rc<Object>, Cons), Error> {
    match &*obj {
        Object::Integer(_)
        | Object::Bool(_)
        | Object::Function(_)
        | Object::BuiltinFunction(_) => Ok((obj, env.clone())),
        Object::Cons(cons) => match cons {
            Cons::Nil => Ok((obj, env.clone())),
            _ => cons.eval(env),
        },
        Object::Symbol(symbol) => symbol.eval(env),
        Object::Quote(quote) => Ok(((*quote).clone(), env.clone())),
    }
}
