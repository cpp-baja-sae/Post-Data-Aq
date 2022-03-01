pub mod data_format;
pub mod hamming;
pub mod ingestion;
pub mod read;
pub mod util;
pub mod read_filtered;

pub fn example_file_descriptor() -> &'static str {
    concat!(
        "0,10000:PackedSwitch,Switches;AccelerometerX,Accelerometer X;",
        "AccelerometerY,Accelerometer Y;AccelerometerZ,AccelerometerZ;",
        "1,2000:MuxCheck0,Check;StrainGauge0,Strain Gauge 0;"
    )
}
