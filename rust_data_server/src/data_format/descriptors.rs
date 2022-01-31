use std::collections::HashMap;

use super::DataType;

pub type DataFrameDescriptor = Vec<DataType>;

/// Checks that every eighth byte is a padding byte so that a hamming code can
/// be added to the data.
pub fn check_hamming_code_compatibility(dfd: &DataFrameDescriptor) -> Result<(), String> {
    let mut byte_counter = 0;
    let mut next_hamming_code_byte = 7;
    for (index, typ) in dfd.iter().enumerate() {
        byte_counter += typ.num_packed_bytes();
        if byte_counter >= next_hamming_code_byte {
            if typ == &DataType::Padding {
                // This should always be true, because the if condition means we just advanced
                // by a single byte, so there is no way for byte_counter >
                // next_hamming_code_byte.
                debug_assert_eq!(byte_counter, next_hamming_code_byte);
                next_hamming_code_byte += 8;
            } else {
                return Err(format!(
                    "The {:?} at index {} is incompatible with a Hamming code",
                    typ, index
                ));
            }
        }
    }
    if byte_counter % 8 == 0 {
        Ok(())
    } else {
        return Err(format!(
            "Data frame size must be a multiple of 8 bytes, got {} instead.",
            byte_counter
        ));
    }
}

#[derive(Clone)]
pub struct PackedFileDescriptor {
    /// The frequency that data frames are collected with.
    sample_rate: f32,
    frame_sequence: Vec<DataFrameDescriptor>,
}

impl PackedFileDescriptor {
    pub fn new(sample_rate: f32, frame_sequence: Vec<DataFrameDescriptor>) -> Self {
        for frame in &frame_sequence {
            debug_assert_eq!(check_hamming_code_compatibility(frame), Ok(()))
        }
        Self {
            sample_rate,
            frame_sequence,
        }
    }

    /// Get a reference to the data file descriptor's sample rate.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Get a reference to the data file descriptor's frame sequence.
    pub fn frame_sequence(&self) -> &[DataFrameDescriptor] {
        &self.frame_sequence[..]
    }

    /// Gets the size in bytes of this descriptor's frame sequence.
    pub fn frame_sequence_size(&self) -> usize {
        let mut frame_sequence_size = 0;
        for frame in self.frame_sequence() {
            for typ in frame {
                frame_sequence_size += typ.num_packed_bytes();
            }
        }
        frame_sequence_size
    }

    /// Creates an UnpackedFileDescriptor which describes unpacked data sourced
    /// from a file with this descriptor.
    pub fn unpacked(&self) -> UnpackedFileDescriptor {
        let mut sample_rate_multipliers = HashMap::<DataType, u8>::new();
        for frame in self.frame_sequence() {
            for typ in frame {
                for typ in typ.unpacked_types() {
                    if sample_rate_multipliers.contains_key(&typ) {
                        *sample_rate_multipliers.get_mut(&typ).unwrap() += 1;
                    } else {
                        sample_rate_multipliers.insert(typ, 1);
                    }
                }
            }
        }

        let base_sample_rate = self.sample_rate / self.frame_sequence.len() as f32;
        let channels = sample_rate_multipliers
            .into_iter()
            .map(|(typ, sample_rate_multiplier)| UnpackedChannelDescriptor {
                typ,
                sample_rate_multiplier,
            })
            .collect();

        UnpackedFileDescriptor {
            sample_rate: base_sample_rate,
            channels,
        }
    }
}

pub struct UnpackedFileDescriptor {
    /// A base frequency that can be multiplied by
    /// `channels[n].sample_rate_multiplier` to get the sample rate for
    /// individual channels.
    pub sample_rate: f32,
    pub channels: Vec<UnpackedChannelDescriptor>,
}

pub struct UnpackedChannelDescriptor {
    pub sample_rate_multiplier: u8,
    pub typ: DataType,
}
