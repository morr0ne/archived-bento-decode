mod decode;
mod encode;

#[cfg(feature = "derive")]
pub use bento_derive::{Bencode, FromBencode, ToBencode};
pub use decode::{Decoder, DecodingError, DictionaryDecoder, FromBencode, ListDecoder, Object};
pub use encode::{DictionaryEncoder, Encoder, ToBencode};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsString(pub Vec<u8>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token<'a> {
    ByteString(&'a [u8]),
    Integer(&'a [u8]),
    ListStart,
    DictionaryStart,
    End,
}

#[derive(Debug)]
pub enum Value<'a> {
    ByteString(&'a [u8]),
    Integer(&'a [u8]),
    List(Vec<Value<'a>>),
    Dictionary(indexmap::IndexMap<&'a [u8], Value<'a>>),
}
