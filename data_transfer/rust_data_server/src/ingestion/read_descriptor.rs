mod data_type;
mod descriptor;
mod utils;

use std::io::Read;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::fail,
    sequence::preceded,
    IResult, Parser,
};

pub use self::descriptor::parse_descriptor;
use crate::data_format::{Axis, DataType, FileDescriptor, PackedDataFrameDescriptor};

pub fn read_descriptor(from: &mut impl Read) -> FileDescriptor {
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
