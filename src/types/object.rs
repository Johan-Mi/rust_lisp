use super::boolean::*;
use super::builtin_function::*;
use super::cons::*;
use super::error::*;
use super::function::*;
use super::integer::*;
use super::quote::*;
use super::symbol::*;
use std::fmt;

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

impl fmt::Display for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(contained) => contained.fmt(formatter),
            Object::Symbol(contained) => contained.fmt(formatter),
            Object::Error(contained) => contained.fmt(formatter),
            Object::Function(contained) => contained.fmt(formatter),
            Object::BuiltinFunction(contained) => contained.fmt(formatter),
            Object::Quote(contained) => contained.fmt(formatter),
            Object::Cons(contained) => contained.fmt(formatter),
            Object::Bool(contained) => contained.fmt(formatter),
        }
    }
}
