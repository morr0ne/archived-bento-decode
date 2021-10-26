pub mod decode;
pub mod encode;
mod token;
pub mod value;

#[derive(Debug)]
pub struct AsString<I>(pub I);
