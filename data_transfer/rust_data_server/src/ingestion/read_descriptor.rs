use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::fail,
    sequence::{preceded, tuple},
    IResult, Parser,
};

use crate::data_format::{Axis, DataType, PackedDataFrameDescriptor, PackedFileDescriptor};

type ParseResult<'a, O> = IResult<&'a str, O>;

fn parse_int(i: &str) -> ParseResult<i32> {
    let (i, int_chars) = take_while1(char::is_numeric)(i)?;
    match int_chars.parse() {
        Ok(int) => Ok((i, int)),
        Err(err) => fail(i),
    }
}

fn parse_colon(i: &str) -> ParseResult<()> {
    let (i, _) = tag(";")(i)?;
    Ok((i, ()))
}

fn parse_semicolon(i: &str) -> ParseResult<()> {
    let (i, _) = tag(";")(i)?;
    Ok((i, ()))
}

fn parse_label(i: &str) -> ParseResult<(u8, i32)> {
    let (i, label) = parse_int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, sample_rate) = parse_int(i)?;
    let (i, _) = parse_colon(i)?;
    Ok((i, (label as u8, sample_rate)))
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

fn parse_item(i: &str) -> ParseResult<(DataType, &str)> {
    let (i, item) = alt((
        parse_accelerometer,
        parse_gps,
        parse_mux_check,
        parse_packed_switch,
        parse_strain_gauge,
    ))(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, label) = take_until(";")(i)?;
    let (i, _) = parse_semicolon(i)?;
    Ok((i, (item, label)))
}

fn parse_descriptor(i: &str) -> ParseResult<PackedFileDescriptor> {
    let (i, _) = parse_semicolon(i)?;
    let mut data_frames = HashMap::new();
    let mut current_data_frame = 0u8;
    let mut i = i;
    while i.len() > 0 {
        if let Ok((new_i, (label, sample_rate))) = parse_label(i) {
            i = new_i;
            current_data_frame = label;
            data_frames.insert(
                current_data_frame,
                PackedDataFrameDescriptor {
                    sample_rate: sample_rate as f32,
                    data_sequence: Vec::new(),
                },
            );
        } else if let Ok((new_i, item)) = parse_item(i) {
            data_frames
                .get_mut(&current_data_frame)
                .unwrap()
                .data_sequence
                .push((item.0, item.1.to_owned()));
        } else {
            return fail(i);
        }
    }
    let descriptor = PackedFileDescriptor::new(data_frames);
    Ok((i, descriptor))
}

pub fn read_descriptor(from: &mut impl Read) -> PackedFileDescriptor {
    let mut size = [0; 4];
    from.read_exact(&mut size)
        .expect("Header size missing in file");
    let size = u32::from_le_bytes(size);
    if size > 10_000 {
        panic!("{} is an unexpectedly large size for a header", size);
    }

    let mut buffer = vec![0; size as usize];
    from.read_exact(&mut buffer)
        .expect("Failed to read header from file");
    let header = String::from_utf8(buffer).expect("Header is not a valid UTF-8 string");

    parse_descriptor(&header).expect("Header is not valid").1
}
