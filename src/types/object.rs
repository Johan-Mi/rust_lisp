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
    Error(Error),
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
            Object::Error(_) => "(type error)",
            Object::Function(_) => "(type function)",
            Object::BuiltinFunction(_) => "(type builtin-function)",
            Object::Quote(_) => "(type quote)",
            Object::Cons(_) => "(type cons)",
            Object::Bool(_) => "(type bool)",
        }
    }
}

pub fn car_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj,
        Object::Cons(cons) => match cons {
            Cons::Some(..) => cons.car(),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("car_obj", &[&*obj]))),
    }
}

pub fn cdr_obj(obj: Rc<Object>) -> Rc<Object> {
    match &*obj {
        Object::Error(_) => obj,
        Object::Cons(cons) => match cons {
            Cons::Some(..) => cons.cdr(),
            Cons::Nil => obj, // We already have a nil, so let's reuse it
        },
        _ => Rc::new(Object::Error(make_type_error("cdr_obj", &[&*obj]))),
    }
}

pub fn apply_obj(
    func_obj: Rc<Object>,
    args: &Cons,
    env: &Cons,
) -> (Rc<Object>, Cons) {
    match &*func_obj {
        Object::Error(_) => (func_obj, env.clone()),
        Object::Function(func) => func.apply(args, env),
        Object::BuiltinFunction(func) => func.apply(args, env),
        _ => (
            Rc::new(Object::Error(make_type_error("apply_obj", &[&*func_obj]))),
            env.clone(),
        ),
    }
}

pub fn eval_obj(obj: Rc<Object>, env: &Cons) -> (Rc<Object>, Cons) {
    match &*obj {
        Object::Error(_)
        | Object::Integer(_)
        | Object::Bool(_)
        | Object::Function(_)
        | Object::BuiltinFunction(_) => (obj, env.clone()),
        Object::Cons(cons) => match cons {
            Cons::Nil => (obj, env.clone()),
            _ => cons.eval(env),
        },
        Object::Symbol(symbol) => symbol.eval(env),
        Object::Quote(quote) => (quote.contained.clone(), env.clone()),
    }
}
