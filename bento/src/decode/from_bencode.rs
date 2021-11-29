use atoi::atoi;
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use super::{AsString, Decoder, DecodingError, Object, Value};

pub trait FromBencode<T: FromBencode = Self> {
    fn from_bencode(bytes: &[u8]) -> Result<T, DecodingError> {
        let mut decoder = Decoder::new(bytes);
        let object = decoder.next_object()?;

        object.map_or(Err(DecodingError::UnexpectedEof), T::decode)
    }

    fn decode(object: Object) -> Result<T, DecodingError>;
}

impl<'obj, 'de> FromBencode for Value<'de> {
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        // Decoder::decode(object.as_bytes())
        todo!()
    }
}

impl FromBencode<Vec<u8>> for AsString {
    fn decode(object: Object) -> Result<Vec<u8>, DecodingError> {
        object.try_byte_string().map(Vec::from)
    }
}

macro_rules! impl_from_bencode_for_num {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn decode(object: Object) -> Result<Self, DecodingError>
            where
                Self: Sized,
            {
                atoi(object.try_integer()?).ok_or(DecodingError::Unknown)
            }
        }
    )*}
}

impl_from_bencode_for_num!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl<T: FromBencode> FromBencode for Vec<T> {
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        let mut list = object.try_list()?;
        let mut results = Vec::new();

        while let Some(object) = list.next_object()? {
            let item = object.decode()?;
            results.push(item);
        }

        Ok(results)
    }
}

impl FromBencode for String {
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        Ok(String::from_utf8(object.try_byte_string()?.to_vec())?)
    }
}

macro_rules! impl_from_bencode_for_from_str {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn decode(object: Object) -> Result<Self, DecodingError>
            where
                Self: Sized,
            {
                object.decode::<String>()?
                    .parse()
                    .map_err(|_| DecodingError::Unknown)
            }
        }
    )*}
}

impl_from_bencode_for_from_str!(Ipv4Addr Ipv6Addr IpAddr SocketAddrV4 SocketAddrV6 SocketAddr );
#[cfg(feature = "url")]
impl_from_bencode_for_from_str!(url::Url);

impl<K, V, H> FromBencode for HashMap<K, V, H>
where
    K: FromBencode + Hash + Eq,
    V: FromBencode,
    H: BuildHasher + Default,
{
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        let mut dict = object.try_dictionary()?;
        let mut result = HashMap::default();

        while let Some((key, value)) = dict.next_pair()? {
            let key = Object::ByteString(key).decode()?;
            let value = value.decode()?;

            result.insert(key, value);
        }

        Ok(result)
    }
}

impl<K, V, H> FromBencode for indexmap::IndexMap<K, V, H>
where
    K: FromBencode + Hash + Eq,
    V: FromBencode,
    H: BuildHasher + Default,
{
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        let mut dict = object.try_dictionary()?;
        let mut result = Self::default();

        while let Some((key, value)) = dict.next_pair()? {
            let key = Object::ByteString(key).decode()?;
            let value = value.decode()?;

            result.insert(key, value);
        }

        Ok(result)
    }
}

impl<T: FromBencode> FromBencode for Option<T> {
    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        object.decode().map(Option::Some)
    }
}
