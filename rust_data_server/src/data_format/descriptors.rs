use super::DataType;

pub type DataFrameDescriptor = Vec<DataType>;

pub struct DataFileDescriptor {
    pub sample_rate: f32,
    pub frame_sequence: Vec<DataFrameDescriptor>,
    pub decoded_data_streams: Vec<DataFrameDescriptor>,
}
