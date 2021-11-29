use std::collections::{LinkedList, VecDeque};

use super::{AsString, Encoder};

pub trait ToBencode {
    fn to_bencode(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        let mut encoder = Encoder::new();

        self.encode(&mut encoder);

        encoder.bytes
    }

    fn encode(&self, encoder: &mut Encoder);
}

// Forwarding impls
impl<'a, E: 'a + ToBencode + Sized> ToBencode for &'a E {
    fn encode(&self, encoder: &mut Encoder) {
        E::encode(self, encoder)
    }
}

// Base type impls
impl<'a> ToBencode for &'a str {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.emit_byte_string(self)
    }
}

impl ToBencode for String {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.emit_byte_string(self)
    }
}

macro_rules! impl_encodable_integer {
    ($($type:ty)*) => {$(
        impl ToBencode for $type {
            fn encode(&self, encoder: &mut Encoder) {
                encoder.emit_integer(*self)
            }
        }
    )*}
}

impl_encodable_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! impl_encodable_iterable {
    ($($type:ident)*) => {$(
        impl <ContentT> ToBencode for $type<ContentT>
        where
            ContentT: ToBencode
        {
            fn encode(&self, encoder: &mut Encoder){
                encoder.emit_list(|e| {
                    for item in self {
                        e.emit(item);
                    }
                });
            }
        }
    )*}
}

impl_encodable_iterable!(Vec VecDeque LinkedList);

impl ToBencode for AsString {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.emit_byte_array(&self.0);
    }
}

impl<'a, T> ToBencode for &'a [T]
where
    T: ToBencode,
{
    fn encode(&self, encoder: &mut Encoder) {
        encoder.emit_list(|e| {
            for item in *self {
                e.emit(item);
            }
        });
    }
}

#[cfg(feature = "url")]
impl ToBencode for url::Url {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.emit_byte_string(self)
    }
}
