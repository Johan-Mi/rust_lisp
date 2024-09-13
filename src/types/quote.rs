use crate::types::Object;
use std::{fmt, rc::Rc};

pub struct Quote(pub Rc<Object>);

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(quote {})", self.0)
    }
}
