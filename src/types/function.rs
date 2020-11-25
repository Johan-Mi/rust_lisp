use super::cons::*;
use super::error::*;
use super::object::*;
use crate::functions::*;
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Display, Constructor)]
#[display(fmt = "Function {} => {}", parameters, body)]
pub struct Function {
    parameters: Cons,
    body: Rc<Object>,
}

impl Function {
    pub fn apply(
        &self,
        args: &Cons,
        env: &Cons,
    ) -> Result<(Rc<Object>, Cons), Error> {
        let (calling_args, env) = eval_list_elements(args, env)?;
        eval_obj(
            self.body.clone(),
            &join_two_lists_cons(&self.parameters, &calling_args, &env),
        )
    }
}
