use derive_more::{Constructor, Display};

#[derive(Display, Constructor)]
#[display(fmt = "Error: {}", message)]
pub struct Error {
    message: String,
}
