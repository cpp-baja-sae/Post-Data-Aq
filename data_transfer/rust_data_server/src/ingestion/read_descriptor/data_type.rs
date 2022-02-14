use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::fail,
    sequence::preceded,
    IResult, Parser,
};

use super::utils::{parse_int, parse_semicolon, ParseResult};
use crate::data_format::{Axis, DataType, FileDescriptor, PackedDataFrameDescriptor};

pub fn parse_data_type(i: &str) -> ParseResult<DataType> {
    alt((
        parse_accelerometer,
        parse_gps,
        parse_mux_check,
        parse_packed_switch,
        parse_padding,
        parse_strain_gauge,
    ))(i)
}

fn parse_accelerometer(i: &str) -> ParseResult<DataType> {
    alt((
        tag("AccelerometerX").map(|_| Axis::X),
        tag("AccelerometerY").map(|_| Axis::Y),
        tag("AccelerometerZ").map(|_| Axis::Z),
    ))
    .map(|axis| DataType::Accelerometer(axis))
    .parse(i)
}

fn parse_gps(i: &str) -> ParseResult<DataType> {
    tag("GPS").map(|_| DataType::Gps).parse(i)
}

fn parse_mux_check(i: &str) -> ParseResult<DataType> {
    preceded(tag("MuxCheck"), parse_int)
        .map(|number| DataType::MuxCheck(number as u8))
        .parse(i)
}

fn parse_padding(i: &str) -> ParseResult<DataType> {
    tag("Padding").map(|_| DataType::Padding).parse(i)
}

fn parse_packed_switch(i: &str) -> ParseResult<DataType> {
    tag("PackedSwitch")
        .map(|_| {
            DataType::PackedSwitch([
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
            ])
        })
        .parse(i)
}

fn parse_strain_gauge(i: &str) -> ParseResult<DataType> {
    preceded(tag("StrainGauge"), parse_int)
        .map(|number| DataType::StrainGauge(number as usize))
        .parse(i)
}
