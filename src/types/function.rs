use super::cons::*;
use super::object::*;
use crate::functions::*;
use std::fmt;
use std::rc::Rc;

pub struct Function {
    pub parameters: Cons,
    pub body: Rc<Object>,
}

impl fmt::Display for Function {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Function {} => {}", self.parameters, self.body)
    }
}

impl Function {
    pub fn apply(&self, args: &Cons, env: &Cons) -> (Rc<Object>, Cons) {
        let (calling_args, env) = eval_list_elements(args, env);
        eval_obj(
            self.body.clone(),
            &join_two_lists_cons(&self.parameters, &calling_args, &env),
        )
    }
}
