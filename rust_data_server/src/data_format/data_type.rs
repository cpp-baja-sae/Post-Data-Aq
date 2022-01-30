use serde::{Deserialize, Serialize};

use super::Axis;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    /// Indicates a f32 value for a particular axis of the accelerometer.
    Accelerometer(Axis),
    /// Indicates a f32 value for a particular axis of the GPS.
    Gps(Axis),
    /// Indicates that a specific byte is expected to indicate the selection
    /// of certain muxes. An error is thrown if this byte is not
    /// exactly found.
    MuxCheck(u8),
    /// Indicates a byte that does not store any information (primarily used
    /// to make room for Hamming codes)
    Padding,
    /// Indicates a f32 value recorded from any strain gague.
    StrainGauge,
    /// Indicates a byte that only stores if a number of switches are open
    /// or closed. Each bit may optionally contain the value of
    /// another switch.
    PackedSwitch([Option<()>; 8]),
    /// Indicates a f32 that only stores if a single switches is closed.
    LoneSwitch,
}

impl DataType {
    /// Returns the types of data streams that data of this type can be
    /// unpacked to.
    pub fn unpacked_types(&self) -> Vec<DataType> {
        match self {
            Self::MuxCheck(..) | Self::Padding => vec![],
            // Count up all bits that contain data for a particular switch.
            Self::PackedSwitch(bit_spec) => bit_spec
                .iter()
                .filter(|b| b.is_some())
                .map(|_| Self::LoneSwitch)
                .collect(),
            // Data type is the same for packed and unpacked representations.
            _ => vec![self.clone()],
        }
    }

    /// Returns the number of bytes that this data type consumes to unpack
    /// {output_streams} values.
    pub fn num_packed_bytes(&self) -> usize {
        match self {
            Self::MuxCheck(..) | Self::Padding | Self::PackedSwitch(..) => 1,
            Self::LoneSwitch => unreachable!("Lone switch is only used for unpacked data"),
            _ => 4,
        }
    }

    /// Unpacks bytes into individual float values with labels.
    pub fn unpack(&self, input: &[u8]) -> Result<Vec<f32>, ()> {
        debug_assert_eq!(input.len(), self.num_packed_bytes());
        let result = match self {
            &DataType::MuxCheck(expecting) => {
                if input[0] == expecting {
                    Ok(vec![])
                } else {
                    Err(())
                }
            }
            DataType::Padding => Ok(vec![]),
            DataType::PackedSwitch(bits) => {
                let mut result = vec![];
                for bit_index in 0..8 {
                    if let Some(..) = bits[bit_index] {
                        let mask = 1 << bit_index;
                        result.push(if input[0] & mask == mask { 1.0 } else { 0.0 });
                    }
                }
                Ok(result)
            }
            DataType::LoneSwitch => unreachable!("Lone switch is only used for unpacked data"),
            _ => Ok(vec![f32::from_le_bytes([
                input[0], input[1], input[2], input[3],
            ])]),
        };
        if let Ok(vec) = &result {
            debug_assert_eq!(vec.len(), self.unpacked_types().len());
        }
        result
    }
}
