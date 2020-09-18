use super::cons::*;
use super::object::*;
use std::fmt;
use std::rc::Rc;

pub struct BuiltinFunction {
    pub func: fn(&Cons, &Cons) -> (Rc<Object>, Cons),
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Builtin function")
    }
}
