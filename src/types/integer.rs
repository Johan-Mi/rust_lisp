use derive_more::{Add, Deref, Display, From, FromStr, Sub};

#[derive(Display, From, FromStr, Deref, Add, Sub, Clone, Copy)]
pub struct Integer(i32);
