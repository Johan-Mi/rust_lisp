use std::fmt;

/// A wrapper around a boolean.
pub struct Bool {
    /// The boolean value contained.
    pub value: bool,
}

impl fmt::Display for Bool {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}
