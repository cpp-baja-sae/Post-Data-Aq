use std::{collections::HashMap, ops::Range};

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

pub struct PackedChannelDescriptor {
    pub sample_rate: f32,
}

#[derive(Clone)]
pub struct FileDescriptor {
    pub data_frame_sample_rates: Vec<f32>,
    /// Which (packed) channels each data frame contains. I.E.
    /// `packed_channel_assignments[data_frame_id]` tells you a range to use on
    /// `packed_channels`
    pub packed_channel_assignments: Vec<Range<usize>>,
    pub packed_channels: Vec<DataType>,
    /// Like `packed_channel_assignments`, but provides ranges to be used on
    /// `unpacked_channels`
    pub unpacked_channel_assignments: Vec<Range<usize>>,
    pub unpacked_channels: Vec<UnpackedChannelDescriptor>,
}

impl FileDescriptor {
    pub fn new(data_frames: Vec<PackedDataFrameDescriptor>) -> Self {
        let mut packed_channel_counter = 0;
        let mut unpacked_channel_counter = 0;
        let mut data_frame_sample_rates = Vec::new();
        let mut packed_channel_assignments = Vec::new();
        let mut packed_channels = Vec::new();
        let mut unpacked_channel_assignments = Vec::new();
        let mut unpacked_channels = Vec::new();

        for frame in data_frames {
            data_frame_sample_rates.push(frame.sample_rate);
            for (typ, name) in frame.data_sequence {
                unpacked_channels.extend(typ.unpacked_types().into_iter().map(|typ| {
                    UnpackedChannelDescriptor {
                        sample_rate: frame.sample_rate,
                        name: name.clone(),
                        typ,
                    }
                }));
                packed_channels.push(typ);
            }
            packed_channel_assignments.push(packed_channel_counter..packed_channels.len());
            packed_channel_counter = packed_channels.len();
            unpacked_channel_assignments.push(unpacked_channel_counter..unpacked_channels.len());
            unpacked_channel_counter = unpacked_channels.len();
        }

        Self {
            data_frame_sample_rates,
            packed_channel_assignments,
            packed_channels,
            unpacked_channel_assignments,
            unpacked_channels,
        }
    }

    /// Creates an UnpackedFileDescriptor which describes unpacked data sourced
    /// from a file with this descriptor.
    pub fn unpacked(&self) -> UnpackedFileDescriptor {
        UnpackedFileDescriptor {
            channels: self.unpacked_channels.clone(),
        }
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
