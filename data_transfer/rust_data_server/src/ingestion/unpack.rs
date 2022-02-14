mod push_bytes;
mod read_frames;

use std::{
    collections::VecDeque,
    io::{self},
};

use super::generate_mips::{self};
use crate::{data_format::FileDescriptor, util::ProgressTracker};

pub trait DataConsumer {
    fn consume(&mut self, datum: f32) -> io::Result<()>;
}

pub struct DataFrameReader<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer> {
    pre_hamming_decode_buffer: VecDeque<u8>,
    post_hamming_decode_buffer: Vec<u8>,
    progress: ProgressTracker,
    descriptor: &'a FileDescriptor,
    progress_callback: ProgressCallback,
    consumers: Vec<Consumer>,
}

pub fn new_file_backed_reader<'a, ProgressCallback: FnMut(u64, u64)>(
    size: u64,
    name: &'a str,
    descriptor: &'a FileDescriptor,
    progress_callback: ProgressCallback,
) -> DataFrameReader<'a, ProgressCallback, impl DataConsumer> {
    let consumers = generate_mips::data_consumers_for(name, descriptor);
    DataFrameReader::new(size, descriptor, progress_callback, consumers)
}

impl<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer>
    DataFrameReader<'a, ProgressCallback, Consumer>
{
    pub fn new(
        size: u64,
        descriptor: &'a FileDescriptor,
        progress_callback: ProgressCallback,
        consumers: Vec<Consumer>,
    ) -> Self {
        assert_eq!(
            descriptor.unpacked_channels.len(),
            consumers.len(),
            "There must be one consumer for every unpacked channel."
        );
        Self {
            pre_hamming_decode_buffer: VecDeque::new(),
            post_hamming_decode_buffer: Vec::new(),
            progress: ProgressTracker::new(size, 1_000_000),
            descriptor,
            progress_callback,
            consumers,
        }
    }
}
