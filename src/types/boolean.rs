use std::fmt;

pub struct Bool {
    pub value: bool,
}

impl fmt::Display for Bool {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.value)
    }
}
