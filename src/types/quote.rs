use super::object::*;
use derive_more::Display;
use std::rc::Rc;

#[derive(Display)]
#[display(fmt = "(quote {})", contained)]
pub struct Quote {
    pub contained: Rc<Object>,
}
