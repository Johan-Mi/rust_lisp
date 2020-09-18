use super::object::*;
use std::fmt;
use std::rc::Rc;

pub struct Quote {
    pub contained: Rc<Object>,
}

impl fmt::Display for Quote {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "(quote {})", self.contained)
    }
}
