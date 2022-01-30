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

pub struct DataFileDescriptor {
    sample_rate: f32,
    frame_sequence: Vec<DataFrameDescriptor>,
}

impl DataFileDescriptor {
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
}
