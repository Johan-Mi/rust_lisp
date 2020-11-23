use derive_more::{Add, Deref, Display, From, FromStr, Sub};

/// A wrapper around an integer.
#[derive(Display, From, FromStr, Deref, Add, Sub, Clone, Copy)]
pub struct Integer(i32);
