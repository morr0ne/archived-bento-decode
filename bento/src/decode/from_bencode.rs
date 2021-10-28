use atoi::atoi;
use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use crate::AsString;

use super::{decoder::Decoder, error::DecodingError, object::Object};

pub trait FromBencode {
    fn from_bencode(bytes: &[u8]) -> Result<Self, DecodingError>
    where
        Self: Sized,
    {
        let mut decoder = Decoder::new(bytes);
        let object = decoder.next_object()?;

        object.map_or(Err(DecodingError::UnexpectedEof), Self::decode)
    }

    fn decode(object: Object) -> Result<Self, DecodingError>
    where
        Self: Sized;
}

impl FromBencode for AsString<Vec<u8>> {
    fn decode(object: Object) -> Result<Self, DecodingError> {
        object.try_byte_string().map(Vec::from).map(AsString)
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
            let item = T::decode(object)?;
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
        // TODO: map proper error
        String::from_utf8(object.try_byte_string()?.to_vec()).map_err(|_| DecodingError::Unknown)
    }
}

macro_rules! impl_from_bencode_for_from_str {
    ($($type:ty)*) => {$(
        impl FromBencode for $type {

            fn decode(object: Object) -> Result<Self, DecodingError>
            where
                Self: Sized,
            {
                String::decode(object)?
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
            let key = K::decode(Object::ByteString(key))?;
            let value = V::decode(value)?;

            result.insert(key, value);
        }

        Ok(result)
    }
}

#[cfg(feature = "indexmap")]
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
            let key = K::decode(Object::ByteString(key))?;
            let value = V::decode(value)?;

            result.insert(key, value);
        }

        Ok(result)
    }
}
