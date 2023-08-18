use std::{borrow::Cow, fmt};

pub struct Error {
    message: Cow<'static, str>,
}

impl Error {
    pub const fn new(message: Cow<'static, str>) -> Self {
        Self { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
