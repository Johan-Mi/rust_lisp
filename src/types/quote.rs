use super::object::*;
use derive_more::{Constructor, Deref, Display};
use std::rc::Rc;

#[derive(Display, Constructor, Deref)]
#[display(fmt = "(quote {})", contained)]
pub struct Quote {
    contained: Rc<Object>,
}
