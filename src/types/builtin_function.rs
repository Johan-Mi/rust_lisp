use super::cons::*;
use super::object::*;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Display, Constructor)]
#[display(fmt = "Builtin function")]
pub struct BuiltinFunction(fn(args: &Cons, env: &Cons) -> (Rc<Object>, Cons));

impl BuiltinFunction {
    pub fn apply(&self, args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
        (self.0)(args, env)
    }
}
