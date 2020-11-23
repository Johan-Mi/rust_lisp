use super::cons::*;
use super::object::*;
use derive_more::Display;
use std::rc::Rc;

#[derive(Display)]
#[display(fmt = "Builtin function")]
pub struct BuiltinFunction {
    pub func: fn(args: &Cons, env: &Cons) -> (Rc<Object>, Cons),
}

impl BuiltinFunction {
    pub fn apply(&self, args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
        (self.func)(args, env)
    }
}
