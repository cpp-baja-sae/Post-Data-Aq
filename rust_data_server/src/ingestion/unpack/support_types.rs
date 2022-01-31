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
