use super::cons::*;
use super::object::*;
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
