use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    io::{self, ErrorKind, Read},
};

use crate::{
    data_format::{
        DataType, PackedFileDescriptor, UnpackedChannelDescriptor, UnpackedFileDescriptor,
    },
    hamming,
    ingestion::read_descriptor::read_descriptor,
};

pub trait DataConsumer {
    fn consume(&mut self, typ: DataType, datum: f32);
}

pub struct DataFrameReader<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer> {
    pre_hamming_decode_buffer: VecDeque<u8>,
    post_hamming_decode_buffer: Vec<u8>,
    pub size: u64,
    pub name: &'a str,
    pub descriptor: &'a PackedFileDescriptor,
    pub progress_callback: ProgressCallback,
    pub consumer: Consumer,
}

enum ReadFrameError {
    CorruptData(String),
    NotEnoughData,
}

impl<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer>
    DataFrameReader<'a, ProgressCallback, Consumer>
{
    pub fn push_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) -> Result<(), String> {
        self.pre_hamming_decode_buffer.extend(bytes);
        while self.pre_hamming_decode_buffer.len() >= 8 {
            let mut chunk: Vec<u8> = self.pre_hamming_decode_buffer.drain(0..8).collect();
            assert_eq!(chunk.len(), 8);
            hamming::decode(chunk.as_mut_slice());
            chunk.pop().unwrap();
            self.post_hamming_decode_buffer.extend(chunk.into_iter());
        }
        loop {
            match self.try_read_frame() {
                Ok(()) => continue,
                Err(ReadFrameError::NotEnoughData) => break Ok(()),
                Err(ReadFrameError::CorruptData(err)) => break Err(err),
            }
        }
    }

    fn try_read_frame(&mut self) -> Result<(), ReadFrameError> {
        let frame_id = *self
            .post_hamming_decode_buffer
            .get(0)
            .ok_or(ReadFrameError::NotEnoughData)?;
        let data_frame = &self.descriptor.data_frames[&frame_id];
        let frame_length = data_frame.packed_len();
        if self.post_hamming_decode_buffer.len() < 1 + frame_length {
            return Err(ReadFrameError::NotEnoughData);
        }
        let data_ptr = &self.post_hamming_decode_buffer[1..];
        for (channel_typ, channel_name) in &data_frame.data_sequence {
            let values = channel_typ
                .unpack(data_ptr)
                .map_err(ReadFrameError::CorruptData)?;
            let unpacked_types = channel_typ.unpacked_types();
            assert_eq!(values.len(), unpacked_types.len());
            for (typ, value) in unpacked_types.into_iter().zip(values.into_iter()) {
                self.consumer.consume(typ, value);
            }
        }
        self.post_hamming_decode_buffer.drain(0..1 + frame_length);
        Ok(())
    }
}
