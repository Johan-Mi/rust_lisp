use crate::{
    functions::{eval_list_elements, join_two_lists_cons},
    types::{Cons, Error, Object},
};
use derive_more::{Constructor, Display};
use std::rc::Rc;

#[derive(Display, Constructor)]
#[display(fmt = "Function {parameters} => {body}")]
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
        self.body.clone().eval(&join_two_lists_cons(
            &self.parameters,
            &calling_args,
            &env,
        ))
    }
}
