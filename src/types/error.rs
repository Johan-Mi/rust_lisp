use std::fmt;

/// An error type that can be returned by Lisp functions if they fail or are
/// called incorrectly.
///
/// # Examples
///
/// ```
/// let err = Error { message: "This is an error" };
/// eprintln!("{}", err);
/// ```
pub struct Error {
    /// A message explaining the error.
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error: {}", self.message)
    }
}
