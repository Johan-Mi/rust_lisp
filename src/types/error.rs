use derive_more::{Constructor, Display};
use std::borrow::Cow;

#[derive(Display, Constructor)]
#[display(fmt = "Error: {}", message)]
pub struct Error {
    message: Cow<'static, str>,
}
