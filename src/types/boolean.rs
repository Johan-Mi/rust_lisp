use derive_more::{Deref, Display, From};

/// A wrapper around a boolean.
#[derive(Display, From, Deref)]
pub struct Bool(bool);
