use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::DataType;

#[derive(Clone)]
pub struct PackedDataFrameDescriptor {
    pub sample_rate: f32,
    pub data_sequence: Vec<(DataType, String)>,
}

impl PackedDataFrameDescriptor {
    pub(crate) fn packed_len(&self) -> usize {
        self.data_sequence
            .iter()
            .map(|(t, _)| t.num_packed_bytes())
            .sum()
    }
}

#[derive(Clone)]
pub struct PackedFileDescriptor {
    pub data_frames: HashMap<u8, PackedDataFrameDescriptor>,
}

impl PackedFileDescriptor {
    pub fn new(data_frames: HashMap<u8, PackedDataFrameDescriptor>) -> Self {
        Self { data_frames }
    }

    /// Creates an UnpackedFileDescriptor which describes unpacked data sourced
    /// from a file with this descriptor.
    pub fn unpacked(&self) -> UnpackedFileDescriptor {
        let mut channels = Vec::new();
        for (_id, frame) in &self.data_frames {
            for (typ, name) in &frame.data_sequence {
                channels.push(UnpackedChannelDescriptor {
                    name: name.clone(),
                    typ: *typ,
                    sample_rate: frame.sample_rate,
                })
            }
        }

        UnpackedFileDescriptor { channels }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnpackedFileDescriptor {
    pub channels: Vec<UnpackedChannelDescriptor>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnpackedChannelDescriptor {
    pub sample_rate: f32,
    pub name: String,
    pub typ: DataType,
}
