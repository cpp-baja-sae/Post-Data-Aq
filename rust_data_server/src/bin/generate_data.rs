use std::{fs::File, io::prelude::*};

use rand::Rng;
use rust_data_server::{data_format::DataType, hamming};

/// Generates a single piece of data with the appropriate type and appends it to
/// the given file.
fn generate_datum(typ: &DataType, append_to: &mut impl Write) {
    let mut rng = rand::thread_rng();
    let mut write = |data| append_to.write_all(data).unwrap();
    match typ {
        DataType::Accelerometer(..) => write(&f32::to_le_bytes(rng.gen_range(-5.0..5.0))),
        DataType::Gps(..) => write(&f32::to_le_bytes(rng.gen_range(-1.0..1.0))),
        DataType::MuxCheck(data) => write(&[*data]),
        DataType::Padding => write(&[0]),
        DataType::StrainGauge => write(&f32::to_le_bytes(rng.gen_range(0.0..1.0))),
        DataType::PackedSwitch(bits) => {
            let mut byte = 0;
            for bit_index in 0..8 {
                if bits[bit_index].is_some() && rng.gen_bool(0.5) {
                    byte |= 1 << bit_index;
                }
            }
            write(&[byte])
        }
        DataType::LoneSwitch => unreachable!("Lone switch is only used for unpacked data"),
    };
}

fn main() {
    let mut file = File::create("data/data.bin").unwrap();
    let descriptor = rust_data_server::example_file_descriptor();
    for i in 0..1_000_000 {
        if i % 10_000 == 0 {
            println!("{} / {}", i / 10_000, 1_000_000 / 10_000);
        }
        let mut buffer = Vec::new();
        for _ in 0..512 {
            for frame in descriptor.frame_sequence() {
                for typ in frame {
                    generate_datum(typ, &mut buffer)
                }
            }
        }
        hamming::encode(&mut buffer[..]);
        file.write_all(&buffer[..]).unwrap();
    }
}
