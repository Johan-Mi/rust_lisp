use super::cons::*;
use super::object::*;
use std::fmt;
use std::rc::Rc;

pub struct BuiltinFunction {
    pub func: fn(args: &Cons, env: &Cons) -> (Rc<Object>, Cons),
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Builtin function")
    }
}

impl BuiltinFunction {
    pub fn apply(&self, args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
        (self.func)(args, env)
    }
}
