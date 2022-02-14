use std::io;

use super::{DataConsumer, DataFrameReader};
use crate::data_format::{DataType, FileDescriptor};

enum ReadFrameError {
    CorruptData(String),
    NotEnoughData,
    Io(io::Error),
}

impl<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer>
    DataFrameReader<'a, ProgressCallback, Consumer>
{
    pub(super) fn read_as_many_frames_as_possible(&mut self) -> Result<(), String> {
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
        let frame_id = self.try_read_frame_id()?;
        let (data_frame, frame_length) = get_packed_frame_info(self.descriptor, frame_id);
        let mut data_ptr = setup_read_ptr(&self.post_hamming_decode_buffer, frame_length)?;
        let mut unpacked_channel_index =
            get_first_unpacked_channel_index(&mut self.descriptor, frame_id);
        for channel_typ in data_frame {
            unpack_datum_from_channel(
                &mut self.consumers,
                channel_typ,
                &mut data_ptr,
                &mut unpacked_channel_index,
            )?;
        }
        // Remove the byte that indicates the frame id, plus all the payload.
        self.post_hamming_decode_buffer.drain(0..1 + frame_length);
        Ok(())
    }

    fn try_read_frame_id(&mut self) -> Result<usize, ReadFrameError> {
        let frame_id = *self
            .post_hamming_decode_buffer
            .get(0)
            .ok_or(ReadFrameError::NotEnoughData)? as usize;
        Ok(frame_id)
    }
}

fn get_first_unpacked_channel_index(
    file_descriptor: &mut &FileDescriptor,
    frame_id: usize,
) -> usize {
    file_descriptor.unpacked_channel_assignments[frame_id].start
}

fn get_packed_frame_info(descriptor: &FileDescriptor, frame_id: usize) -> (&[DataType], usize) {
    let data_frame = get_packed_channels(descriptor, frame_id);
    let frame_length: usize = data_frame.iter().map(DataType::num_packed_bytes).sum();
    (data_frame, frame_length)
}

fn get_packed_channels(descriptor: &FileDescriptor, frame_id: usize) -> &[DataType] {
    let channels = descriptor.packed_channel_assignments[frame_id].clone();
    let data_frame = &descriptor.packed_channels[channels];
    data_frame
}

fn setup_read_ptr(input_buffer: &[u8], frame_length: usize) -> Result<&[u8], ReadFrameError> {
    if input_buffer.len() < 1 + frame_length {
        return Err(ReadFrameError::NotEnoughData);
    }
    let data_ptr = &input_buffer[1..];
    Ok(data_ptr)
}

fn unpack_datum_from_channel(
    consumers: &mut [impl DataConsumer],
    channel_typ: &DataType,
    data_ptr: &mut &[u8],
    unpacked_channel_index: &mut usize,
) -> Result<(), ReadFrameError> {
    let values = channel_typ
        .unpack(data_ptr)
        .map_err(ReadFrameError::CorruptData)?;
    *data_ptr = &data_ptr[channel_typ.num_packed_bytes()..];
    write_datum_to_unpacked_channels(values, consumers, unpacked_channel_index)?;
    Ok(())
}

fn write_datum_to_unpacked_channels(
    values: Vec<f32>,
    consumers: &mut [impl DataConsumer],
    unpacked_channel_index: &mut usize,
) -> Result<(), ReadFrameError> {
    for value in values.into_iter() {
        consumers[*unpacked_channel_index]
            .consume(value)
            .map_err(ReadFrameError::Io)?;
        *unpacked_channel_index += 1;
    }
    Ok(())
}
