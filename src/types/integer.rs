use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

/// A wrapper around an integer.
pub struct Integer {
    /// The integer value contained.
    pub value: i32,
}

impl FromStr for Integer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Integer { value: s.parse()? })
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}
