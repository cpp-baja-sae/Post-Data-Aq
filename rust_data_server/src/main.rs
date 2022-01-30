use secded::SecDedCodec;

pub type HammingCode = secded::SecDed64;

const CACHE_DIR: &'static str = "cache";

// Cannot be more than 57.
const BITS_PER_PAYLOAD: usize = 57;

fn hamming_decode(input: &mut [u8]) -> Result<(), ()> {
    let hc = HammingCode::new(BITS_PER_PAYLOAD);
    hc.decode(input)
}

#[derive(Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub use Axis::{X as Longitude, Y as Latitude, Z as Elevation};

#[derive(Clone, Debug)]
pub enum DataType {
    /// Indicates a f32 value for a particular axis of the accelerometer.
    Accelerometer(Axis),
    /// Indicates a f32 value for a particular axis of the GPS.
    Gps(Axis),
    /// Indicates that a specific byte is expected to indicate the selection of
    /// certain muxes. An error is thrown if this byte is not exactly found.
    MuxCheck(u8),
    /// Indicates a byte that does not store any information (primarily used to
    /// make room for Hamming codes)
    Padding,
    /// Indicates a f32 value recorded from any strain gague.
    StrainGauge,
    /// Indicates a byte that only stores if a number of switches are open or
    /// closed. Each bit may optionally contain the value of another switch.
    PackedSwitch([Option<()>; 8]),
    /// Indicates a f32 that only stores if a single switches is closed.
    LoneSwitch,
}

pub type DataFrameDescriptor = Vec<DataType>;

pub struct DataFileDescriptor {
    pub sample_rate: f32,
    pub frame_sequence: Vec<DataFrameDescriptor>,
    pub decoded_data_streams: Vec<DataFrameDescriptor>,
}

fn main() {}
