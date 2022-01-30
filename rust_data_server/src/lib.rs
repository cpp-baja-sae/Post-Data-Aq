pub mod data_format;
pub mod hamming;

use data_format::{Axis, PackedFileDescriptor, DataType};

pub fn example_file_descriptor() -> PackedFileDescriptor {
    use DataType::*;
    let switch_descriptor =
        PackedSwitch([None, None, None, None, Some(0), None, Some(1), Some(2)]);
    PackedFileDescriptor::new(
        1e4,
        vec![
            vec![
                switch_descriptor.clone(),
                MuxCheck(0),
                StrainGauge(0),
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(1),
                StrainGauge(1),
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(2),
                StrainGauge(2),
                Padding,
                Padding,
            ],
            vec![
                switch_descriptor.clone(),
                MuxCheck(3),
                StrainGauge(3),
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
