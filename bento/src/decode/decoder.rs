use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    combinator::{map, map_parser, map_res, opt, peek, rest},
    multi::{fold_many0, length_data, many0},
    sequence::{delimited, pair, terminated, tuple},
    Finish, IResult,
};

use super::{DecodingError, Object, Token, Value};

pub struct Decoder<'a> {
    bytes: &'a [u8],
}
pub struct ListDecoder<'obj, 'de: 'obj>(&'obj mut Decoder<'de>);
pub struct DictionaryDecoder<'obj, 'de: 'obj>(&'obj mut Decoder<'de>);

impl<'de> Decoder<'de> {
    pub const fn new(bytes: &'de [u8]) -> Self {
        Self { bytes }
    }

    /// Decodes a byte string without wrapping it into a Token.
    pub fn decode_byte_string_raw(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
        length_data(map_parser(
            terminated(digit1, char(':')),
            nom::character::complete::u64,
        ))(bytes)
    }

    /// Returns a [byte string](Token::ByteString)
    pub fn decode_byte_string_token(bytes: &[u8]) -> IResult<&[u8], Token> {
        map(Self::decode_byte_string_raw, Token::ByteString)(bytes)
    }

    /// Returns a [byte string](Token::ByteString)
    pub fn decode_byte_string(bytes: &[u8]) -> IResult<&[u8], Value> {
        map(Self::decode_byte_string_raw, Value::ByteString)(bytes)
    }

    // TODO: this function def needs some optimization
    fn decode_integer_raw(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
        map_parser(
            delimited(char('i'), take_until("e"), char('e')),
            map_res(
                tuple((peek(tuple((opt(tag(b"-")), digit1))), rest)),
                |((sign, _integer), complete)| {
                    if let Some(sign) = sign {
                        if sign != b"-" && complete == b"-0" {
                            Err(DecodingError::Unknown)
                        } else {
                            Ok(complete)
                        }
                    } else {
                        Ok(complete)
                    }
                },
            ),
        )(bytes)
    }

    /// Returns an integer leaving it into its byte form.
    fn decode_integer_token(bytes: &[u8]) -> IResult<&[u8], Token> {
        map(Self::decode_integer_raw, Token::Integer)(bytes)
    }

    /// Returns an integer leaving it into its byte form.
    fn decode_integer(bytes: &[u8]) -> IResult<&[u8], Value> {
        map(Self::decode_integer_raw, Value::Integer)(bytes)
    }

    /// Decodes a list directly returning a value instead of a token
    pub fn decode_list(bytes: &[u8]) -> IResult<&[u8], Value> {
        map(
            delimited(char('l'), many0(Self::decode_any), char('e')),
            Value::List,
        )(bytes)
    }

    /// Decodes a dictionary directly returning a value instead of a token
    pub fn decode_dictionaries(bytes: &[u8]) -> IResult<&[u8], Value> {
        map(
            delimited(
                char('d'),
                fold_many0(
                    pair(Self::decode_byte_string_raw, Self::decode_any),
                    IndexMap::new,
                    |mut dict, (key, value)| {
                        dict.insert(key, value);
                        dict
                    },
                ),
                char('e'),
            ),
            Value::Dictionary,
        )(bytes)
    }

    pub fn decode_any(bytes: &[u8]) -> IResult<&[u8], Value> {
        alt((
            Self::decode_byte_string,
            Self::decode_integer,
            Self::decode_list,
            Self::decode_dictionaries,
        ))(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Value, DecodingError> {
        Self::decode_any(bytes)
            .finish()
            .map(|(_rest, value)| value)
            .map_err(|_error| DecodingError::Unknown)
    }

    fn next_token(&mut self) -> Result<Option<Token<'de>>, DecodingError> {
        alt((
            Self::decode_byte_string_token,
            Self::decode_integer_token,
            map(char('l'), |_| Token::ListStart),
            map(char('d'), |_| Token::DictionaryStart),
            map(char('e'), |_| Token::End),
        ))(self.bytes)
        .finish()
        .map(|(bytes, token)| {
            self.bytes = bytes;
            Some(token)
        })
        .map_err(|_error| DecodingError::Unknown) // TODO: Map to an actual error
    }

    pub fn next_object<'obj>(&'obj mut self) -> Result<Option<Object<'obj, 'de>>, DecodingError> {
        Ok(match self.next_token()? {
            None | Some(Token::End) => None,
            Some(Token::ByteString(byte_string)) => Some(Object::ByteString(byte_string)),
            Some(Token::Integer(integer)) => Some(Object::Integer(integer)),
            Some(Token::ListStart) => Some(Object::List(ListDecoder::new(self))),
            Some(Token::DictionaryStart) => Some(Object::Dictionary(DictionaryDecoder::new(self))),
        })
    }
}

impl<'obj, 'de: 'obj> ListDecoder<'obj, 'de> {
    pub const fn as_bytes(self) -> &'de [u8] {
        self.0.bytes
    }

    pub fn new(decoder: &'obj mut Decoder<'de>) -> Self {
        Self(decoder)
    }

    pub fn next_object<'item>(
        &'item mut self,
    ) -> Result<Option<Object<'item, 'de>>, DecodingError> {
        self.0.next_object()
    }
}

impl<'obj, 'de: 'obj> DictionaryDecoder<'obj, 'de> {
    pub const fn as_bytes(self) -> &'de [u8] {
        self.0.bytes
    }

    pub fn new(decoder: &'obj mut Decoder<'de>) -> Self {
        Self(decoder)
    }

    pub fn next_pair<'item>(
        &'item mut self,
    ) -> Result<Option<(&'de [u8], Object<'item, 'de>)>, DecodingError> {
        if let Some(Object::ByteString(key)) = self.0.next_object()? {
            if let Some(value) = self.0.next_object()? {
                Ok(Some((key, value)))
            } else {
                Err(DecodingError::MissingDictionaryValue)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;
    use nom::{Err as NomErr, Needed};

    #[test]
    fn decode_integer() {
        assert_eq!(
            Ok(([].as_ref(), b"1".as_ref())),
            Decoder::decode_integer_raw(b"i1e")
        );
    }

    #[test]
    fn decode_negative_integer() {
        assert_eq!(
            Ok(([].as_ref(), b"-1".as_ref())),
            Decoder::decode_integer_raw(b"i-1e")
        );
    }

    #[test]
    fn decode_negative_zero() {
        assert_eq!(
            Ok(([].as_ref(), b"-0".as_ref())),
            Decoder::decode_integer_raw(b"i-0e")
        );
    }

    #[test]
    fn decode_big_integer() {
        assert_eq!(
            Ok(([].as_ref(), b"02398421923842".as_ref())),
            Decoder::decode_integer_raw(b"i02398421923842e")
        );
    }

    #[test]
    fn decode_byte_string() {
        assert_eq!(
            Ok(([].as_ref(), b"hello".as_ref())),
            Decoder::decode_byte_string_raw(b"5:hello")
        );
    }

    #[test]
    fn decode_byte_string_invalid_len() {
        assert_eq!(
            Err(NomErr::Incomplete(Needed::Size(unsafe {
                NonZeroUsize::new_unchecked(1)
            }))), // Hello is of len 5, that means we get an error that we are missing 1 more byte
            Decoder::decode_byte_string_raw(b"6:hello")
        );
    }
}
