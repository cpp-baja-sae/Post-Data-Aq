use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::fail,
    IResult, Parser,
};

pub type ParseResult<'a, O> = IResult<&'a str, O>;

pub fn parse_int(i: &str) -> ParseResult<i32> {
    let (i, int_chars) = take_while1(char::is_numeric)(i)?;
    match int_chars.parse() {
        Ok(int) => Ok((i, int)),
        Err(..) => fail(i),
    }
}

pub fn parse_colon(i: &str) -> ParseResult<()> {
    let (i, _) = tag(":")(i)?;
    Ok((i, ()))
}

pub fn parse_semicolon(i: &str) -> ParseResult<()> {
    let (i, _) = tag(";")(i)?;
    Ok((i, ()))
}
