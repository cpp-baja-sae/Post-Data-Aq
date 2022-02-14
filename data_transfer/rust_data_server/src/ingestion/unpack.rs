use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    io::{self, ErrorKind, Read},
};

use super::generate_mips::{self, RootDataConsumer};
use crate::{
    data_format::{DataType, FileDescriptor, UnpackedChannelDescriptor, UnpackedFileDescriptor},
    hamming,
    ingestion::read_descriptor::read_descriptor,
};

pub trait DataConsumer {
    fn consume(&mut self, datum: f32) -> io::Result<()>;
}

pub struct DataFrameReader<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer> {
    pre_hamming_decode_buffer: VecDeque<u8>,
    post_hamming_decode_buffer: Vec<u8>,
    size: u64,
    progress: u64,
    last_notification: u64,
    descriptor: &'a FileDescriptor,
    progress_callback: ProgressCallback,
    consumers: Vec<Consumer>,
}

enum ReadFrameError {
    CorruptData(String),
    NotEnoughData,
    Io(io::Error),
}

pub fn new_file_backed<'a, ProgressCallback: FnMut(u64, u64)>(
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
            size,
            progress: 0,
            last_notification: 0,
            descriptor,
            progress_callback,
            consumers,
        }
    }

    pub fn push_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) -> Result<(), String> {
        let old_len = self.pre_hamming_decode_buffer.len();
        self.pre_hamming_decode_buffer.extend(bytes);
        let extended_by = self.pre_hamming_decode_buffer.len() - old_len;
        self.progress += extended_by as u64;
        if self.progress - self.last_notification >= 1_000_000 {
            self.last_notification = self.progress;
            (self.progress_callback)(self.progress, self.size);
        }
        while self.pre_hamming_decode_buffer.len() >= 8 {
            let mut chunk: Vec<u8> = self.pre_hamming_decode_buffer.drain(0..8).collect();
            assert_eq!(chunk.len(), 8);
            hamming::decode(chunk.as_mut_slice())
                .map_err(|_| format!("Error detected during Hamming decode"))?;
            // Remove last byte that was used to store a hamming code.
            chunk.pop().unwrap();
            self.post_hamming_decode_buffer.extend(chunk.into_iter());
        }
        loop {
            match self.try_read_frame() {
                Ok(()) => continue,
                Err(ReadFrameError::NotEnoughData) => break Ok(()),
                Err(ReadFrameError::CorruptData(err)) => break Err(err),
                Err(ReadFrameError::Io(err)) => break Err(err.to_string()),
            }
        }
    }

    fn try_read_frame(&mut self) -> Result<(), ReadFrameError> {
        let frame_id = *self
            .post_hamming_decode_buffer
            .get(0)
            .ok_or(ReadFrameError::NotEnoughData)? as usize;
        let channels = self.descriptor.packed_channel_assignments[frame_id].clone();
        let data_frame = &self.descriptor.packed_channels[channels];
        let frame_length: usize = data_frame.iter().map(DataType::num_packed_bytes).sum();
        if self.post_hamming_decode_buffer.len() < 1 + frame_length {
            return Err(ReadFrameError::NotEnoughData);
        }
        let data_ptr = &self.post_hamming_decode_buffer[1..];
        let unpacked_channels = self.descriptor.unpacked_channel_assignments[frame_id].clone();
        let mut unpacked_channel_index = unpacked_channels.start;
        for channel_typ in data_frame {
            let values = channel_typ
                .unpack(data_ptr)
                .map_err(ReadFrameError::CorruptData)?;
            for value in values.into_iter() {
                self.consumers[unpacked_channel_index]
                    .consume(value)
                    .map_err(ReadFrameError::Io)?;
                unpacked_channel_index += 1;
            }
        }
        self.post_hamming_decode_buffer.drain(0..1 + frame_length);
        Ok(())
    }
}
