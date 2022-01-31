use std::{collections::HashMap, fs::File, io::BufWriter};

use crate::data_format::DataType;

pub(super) type Outputs = Vec<(DataType, BufWriter<File>)>;
pub(super) type ReadSequence<'a> = Vec<ReadStep<'a>>;
pub(super) type SampleRateInfo = HashMap<DataType, u8>;

pub(super) struct ReadStep<'a> {
    pub(super) num_bytes: usize,
    pub(super) unpacker: &'a DataType,
    /// Indexes into an instance of Outputs, says which output each unpacked
    /// value should be written to.
    pub(super) destinations: Vec<usize>,
}

pub(super) struct ProgressTracker {
    pub(super) total_bytes: u64,
    pub(super) bytes_so_far: u64,
    pub(super) last_notification: u64,
}

impl ProgressTracker {
    pub fn advance(&mut self, amount: u64, progress_callback: &mut impl FnMut(u64, u64)) {
        self.bytes_so_far += amount;
        if self.bytes_so_far - self.last_notification > 10_000_000 {
            progress_callback(self.bytes_so_far, self.total_bytes);
            self.last_notification = self.bytes_so_far;
        }
    }
}
