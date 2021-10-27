use super::{
    decoder::{DictionaryDecoder, ListDecoder},
    DecodingError,
};

pub enum Object<'obj, 'de: 'obj> {
    ByteString(&'de [u8]),
    Integer(&'de [u8]),
    List(ListDecoder<'obj, 'de>),
    Dictionary(DictionaryDecoder<'obj, 'de>),
}

impl<'obj, 'de: 'obj> Object<'obj, 'de> {
    pub const fn byte_string(self) -> Option<&'de [u8]> {
        match self {
            Object::ByteString(byte_string) => Some(byte_string),
            _ => None,
        }
    }

    pub const fn try_byte_string(self) -> Result<&'de [u8], DecodingError> {
        match self {
            Object::ByteString(byte_string) => Ok(byte_string),
            _ => Err(DecodingError::Unknown),
        }
    }

    pub const fn integer(self) -> Option<&'de [u8]> {
        match self {
            Object::Integer(integer) => Some(integer),
            _ => None,
        }
    }

    pub const fn try_integer(self) -> Result<&'de [u8], DecodingError> {
        match self {
            Object::Integer(integer) => Ok(integer),
            _ => Err(DecodingError::Unknown),
        }
    }

    pub const fn list(self) -> Option<ListDecoder<'obj, 'de>> {
        match self {
            Object::List(list_decoder) => Some(list_decoder),
            _ => None,
        }
    }

    pub const fn try_list(self) -> Result<ListDecoder<'obj, 'de>, DecodingError> {
        match self {
            Object::List(list_decoder) => Ok(list_decoder),
            _ => Err(DecodingError::Unknown),
        }
    }

    pub const fn dictionary(self) -> Option<DictionaryDecoder<'obj, 'de>> {
        match self {
            Object::Dictionary(dictionary_decoder) => Some(dictionary_decoder),
            _ => None,
        }
    }

    pub const fn try_dictionary(self) -> Result<DictionaryDecoder<'obj, 'de>, DecodingError> {
        match self {
            Object::Dictionary(dictionary_decoder) => Ok(dictionary_decoder),
            _ => Err(DecodingError::Unknown),
        }
    }
}
