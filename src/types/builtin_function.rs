use super::cons::*;
use super::error::*;
use super::object::*;
use derive_more::{Constructor, Display};
use std::rc::Rc;

type FnType = fn(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons), Error>;

#[derive(Display, Constructor)]
#[display(fmt = "Builtin function")]
pub struct BuiltinFunction(FnType);

impl BuiltinFunction {
    pub fn apply(
        &self,
        args: &Cons,
        env: &Cons,
    ) -> Result<(Rc<Object>, Cons), Error> {
        (self.0)(args, env)
    }
}
