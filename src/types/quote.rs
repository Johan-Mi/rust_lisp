use crate::types::Object;
use std::{fmt, rc::Rc};

pub struct Quote {
    contained: Rc<Object>,
}

impl std::ops::Deref for Quote {
    type Target = Rc<Object>;

    fn deref(&self) -> &Self::Target {
        &self.contained
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(quote {})", self.contained)
    }
}

impl Quote {
    pub fn new(contained: Rc<Object>) -> Self {
        Self { contained }
    }
}
