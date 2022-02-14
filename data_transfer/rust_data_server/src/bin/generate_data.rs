use std::{
    fs::{self, File},
    io::{prelude::*, BufWriter},
};

use rand::Rng;
use rust_data_server::{data_format::DataType, hamming};

/// Generates a single piece of data with the appropriate type and appends it to
/// the given file.
fn generate_datum(typ: &DataType, append_to: &mut impl Write) {
    let mut rng = rand::thread_rng();
    let mut write = |data| append_to.write_all(data).unwrap();
    match typ {
        DataType::Accelerometer(..) => write(&f32::to_le_bytes(rng.gen_range(-5.0..5.0))),
        DataType::Gps => todo!(),
        DataType::MuxCheck(data) => write(&[*data]),
        DataType::Padding => write(&[0]),
        DataType::StrainGauge(..) => write(&f32::to_le_bytes(rng.gen_range(0.0..1.0))),
        DataType::PackedSwitch(bits) => {
            let mut byte = 0;
            for bit_index in 0..8 {
                if bits[bit_index].is_some() && rng.gen_bool(0.5) {
                    byte |= 1 << bit_index;
                }
            }
            write(&[byte])
        }
        DataType::LoneSwitch(..) => unreachable!("Lone switch is only used for unpacked data"),
    };
}

fn main() {
    fs::create_dir_all("data/").unwrap();
    let file = File::create("data/sample.bin").unwrap();
    let mut file = BufWriter::new(file);
    let descriptor = rust_data_server::example_file_descriptor();
    file.write_all(&(descriptor.len() as u32).to_le_bytes())
        .unwrap();
    file.write_all(descriptor.as_bytes()).unwrap();
    let descriptor = rust_data_server::ingestion::parse_descriptor(descriptor);
    let descriptor = descriptor.unwrap().1;
    let mut time = 0.0f64;
    let mut last_times_per_frame_type = vec![0.0; descriptor.packed_channels.len()];
    let mut buffer = Vec::new();
    for i in 0..50_000_000 {
        if i % 10_000 == 0 {
            println!("{} / {}", i / 10_000, 50_000_000 / 10_000);
        }
        time += 1.0 / 10_000.0;
        for (index, &freq) in descriptor.data_frame_sample_rates.iter().enumerate() {
            let interval = (1.0 / freq) as f64;
            if time - last_times_per_frame_type[index] >= interval {
                last_times_per_frame_type[index] += interval;
                buffer.push(index as u8);
                let channels = descriptor.packed_channel_assignments[index].clone();
                for channel in &descriptor.packed_channels[channels] {
                    generate_datum(channel, &mut buffer);
                }
            }
        }
        while buffer.len() >= 8 {
            let reserve_byte = std::mem::take(&mut buffer[7]);
            hamming::encode(&mut buffer[..8]);
            file.write_all(&buffer[..8]).unwrap();
            buffer.drain(0..7);
            buffer[0] = reserve_byte;
        }
    }
}
