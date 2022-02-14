use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::fail,
    sequence::preceded,
    IResult, Parser,
};

use super::{
    data_type::parse_data_type,
    utils::{parse_int, parse_semicolon, ParseResult, parse_colon},
};
use crate::data_format::{Axis, DataType, FileDescriptor, PackedDataFrameDescriptor};

pub fn parse_descriptor(i: &str) -> ParseResult<FileDescriptor> {
    let mut data_frames = Vec::new();
    let mut current_data_frame = None;
    let mut i = i;
    while i.len() > 0 {
        i = parse_item_or_label(i, &mut current_data_frame, &mut data_frames)?.0;
    }
    let descriptor = FileDescriptor::new(data_frames);
    Ok((i, descriptor))
}

fn parse_item(i: &str) -> ParseResult<(DataType, &str)> {
    let (i, item) = parse_data_type(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, label) = take_until(";")(i)?;
    let (i, _) = parse_semicolon(i)?;
    Ok((i, (item, label)))
}

pub fn parse_label(i: &str) -> ParseResult<(u8, i32)> {
    let (i, label) = parse_int(i)?;
    let (i, _) = tag(",")(i)?;
    let (i, sample_rate) = parse_int(i)?;
    let (i, _) = parse_colon(i)?;
    Ok((i, (label as u8, sample_rate)))
}

fn parse_item_or_label<'i>(
    i: &'i str,
    current_data_frame: &mut Option<u8>,
    data_frames: &mut Vec<PackedDataFrameDescriptor>,
) -> ParseResult<'i, ()> {
    if let Ok((i, (label, sample_rate))) = parse_label(i) {
        append_new_data_frame(current_data_frame, data_frames, label, sample_rate);
        Ok((i, ()))
    } else if let Ok((i, item)) = parse_item(i) {
        append_item_to_last_data_frame(data_frames, item);
        Ok((i, ()))
    } else {
        fail(i)
    }
}

fn append_new_data_frame(
    current_data_frame: &mut Option<u8>,
    data_frames: &mut Vec<PackedDataFrameDescriptor>,
    label: u8,
    sample_rate: i32,
) {
    let expected_label = current_data_frame.map(|x| x + 1).unwrap_or(0);
    assert_eq!(expected_label, label, "Unexpected jump in data frame label");
    *current_data_frame = Some(label);
    data_frames.push(PackedDataFrameDescriptor {
        sample_rate: sample_rate as f32,
        data_sequence: Vec::new(),
    });
}

fn append_item_to_last_data_frame(
    data_frames: &mut Vec<PackedDataFrameDescriptor>,
    item: (DataType, &str),
) {
    data_frames
        .last_mut()
        .expect("Unexpected item before any label")
        .data_sequence
        .push((item.0, item.1.to_owned()));
}
