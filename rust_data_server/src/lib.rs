pub mod data_format;
pub mod hamming;

use data_format::{Axis, DataFileDescriptor, DataType};

pub fn example_file_descriptor() -> DataFileDescriptor {
    use DataType::*;
    let switch_descriptor =
        PackedSwitch([None, None, None, None, Some(()), None, Some(()), Some(())]);
    DataFileDescriptor::new(
        1e4,
        vec![
            vec![
                switch_descriptor.clone(),
                MuxCheck(0),
                StrainGauge,
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(1),
                StrainGauge,
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(2),
                StrainGauge,
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(3),
                StrainGauge,
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(4),
                Accelerometer(Axis::X),
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(5),
                Accelerometer(Axis::Y),
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(6),
                Accelerometer(Axis::Z),
                Padding,
                Padding,
            ],
        ],
    )
}
