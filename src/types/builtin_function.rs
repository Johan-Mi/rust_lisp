use crate::types::{Cons, Object};
use anyhow::Result;
use std::{fmt, rc::Rc};

type FnType = fn(args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)>;

pub struct BuiltinFunction(pub FnType);

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Builtin function")
    }
}

impl BuiltinFunction {
    pub fn apply(&self, args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
        (self.0)(args, env)
    }
}
