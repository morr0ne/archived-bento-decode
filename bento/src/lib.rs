mod decode;
mod encode;

pub use decode::{Decoder, DecodingError, DictionaryDecoder, FromBencode, ListDecoder, Object};
pub use encode::{DictionaryEncoder, Encoder, ToBencode};
#[derive(Debug)]
pub struct AsString<I>(pub I);

pub(crate) enum Token<'a> {
    ByteString(&'a [u8]),
    Integer(&'a [u8]),
    ListStart,
    DictionaryStart,
    End,
}
