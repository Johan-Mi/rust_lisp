use derive_more::{Deref, Display, From};

#[derive(Display, From, Deref)]
pub struct Bool(bool);
