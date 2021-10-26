use indexmap::IndexMap;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char, digit1},
    combinator::{map, map_parser},
    multi::{fold_many0, length_data, many0},
    sequence::{delimited, pair, terminated},
    Finish, IResult,
};

#[derive(Debug)]
pub enum Value<'a> {
    ByteString(&'a [u8]),
    Integer(&'a [u8]),
    List(Vec<Value<'a>>),
    Dictionary(IndexMap<&'a [u8], Value<'a>>),
}

pub trait Bencode {
    fn bdecode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Self::from_value(decode(bytes).unwrap())
    }

    fn from_value(value: Value) -> Result<Self>
    where
        Self: Sized;
}

pub fn decode(bytes: &[u8]) -> Result<Value, nom::error::Error<&[u8]>> {
    Ok(decode_any(bytes).finish()?.1)
}

pub fn decode_any(bytes: &[u8]) -> IResult<&[u8], Value> {
    alt((
        decode_byte_string,
        decode_integer,
        decode_lists,
        decode_dictionaries,
    ))(bytes)
}

pub fn decode_byte_string_raw(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    length_data(map_parser(
        terminated(digit1, char(':')),
        nom::character::complete::u64,
    ))(bytes)
}

pub fn decode_byte_string(bytes: &[u8]) -> IResult<&[u8], Value> {
    map(decode_byte_string_raw, Value::ByteString)(bytes)
}

pub fn decode_integer(bytes: &[u8]) -> IResult<&[u8], Value> {
    map(
        delimited(char('i'), take_until("e"), char('e')),
        Value::Integer,
    )(bytes)
}

pub fn decode_lists(bytes: &[u8]) -> IResult<&[u8], Value> {
    map(
        delimited(char('l'), many0(decode_any), char('e')),
        Value::List,
    )(bytes)
}

pub fn decode_dictionaries(bytes: &[u8]) -> IResult<&[u8], Value> {
    map(
        delimited(
            char('d'),
            fold_many0(
                pair(decode_byte_string_raw, decode_any),
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
