use crate::{
    functions::{eval_list_elements, join_two_lists_cons},
    types::{Cons, Object},
};
use anyhow::Result;
use std::{fmt, rc::Rc};

pub struct Function {
    parameters: Cons,
    body: Rc<Object>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function {} => {}", self.parameters, self.body)
    }
}

impl Function {
    pub const fn new(parameters: Cons, body: Rc<Object>) -> Self {
        Self { parameters, body }
    }

    pub fn apply(&self, args: &Cons, env: &Cons) -> Result<(Rc<Object>, Cons)> {
        let (calling_args, env) = eval_list_elements(args, env)?;
        self.body.clone().eval(&join_two_lists_cons(
            &self.parameters,
            &calling_args,
            &env,
        ))
    }
}
