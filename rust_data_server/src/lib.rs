pub mod data_format;
pub mod hamming;
pub mod ingestion;

use data_format::{Axis, DataType, PackedFileDescriptor};

pub mod util {
    pub struct ProgressTracker {
        total_bytes: u64,
        bytes_so_far: u64,
        last_notification: u64,
        update_interval: u64,
    }

    impl ProgressTracker {
        pub fn new(total_bytes: u64, update_interval: u64) -> Self {
            Self {
                total_bytes,
                bytes_so_far: 0,
                last_notification: 9,
                update_interval,
            }
        }

        pub fn advance(&mut self, amount: u64, progress_callback: &mut impl FnMut(u64, u64)) {
            self.bytes_so_far += amount;
            if self.bytes_so_far - self.last_notification >= self.update_interval {
                progress_callback(self.bytes_so_far, self.total_bytes);
                self.last_notification = self.bytes_so_far;
            }
        }
    }
}

pub fn example_file_descriptor() -> PackedFileDescriptor {
    use DataType::*;
    let switch_descriptor = PackedSwitch([None, None, None, None, Some(0), None, Some(1), Some(2)]);
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
