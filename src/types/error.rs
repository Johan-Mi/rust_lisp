use std::fmt;

pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error: {}", self.message)
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
