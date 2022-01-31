use std::{fs::File, io::{BufReader, BufWriter, Read, Write}};

use crate::{
    data_format::{DataType, PackedFileDescriptor},
    util::ProgressTracker,
};

pub fn generate_mips(
    name: &str,
    descriptor: &PackedFileDescriptor,
    mut progress_callback: impl FnMut(u64, u64),
) {
    let descriptor = descriptor.unpacked();
    let data_dir = format!("cache/{}", name);

    let mut total_bytes = 0;
    for channel in &descriptor.channels {
        let path = file_name(&file_name_base(&data_dir, &channel.typ), 0, "avg");
        let metadata = File::open(path).unwrap().metadata().unwrap();
        total_bytes += metadata.len();
    }
    // The total number of bytes we need to process approaches 6 times the original
    // data size as the number of mips goes to infinity.
    total_bytes *= 6;
    let mut progress = ProgressTracker::new(total_bytes, 100_000_000);

    for target_mip_index in 1..40 {
        for channel in &descriptor.channels {
            let base = file_name_base(&data_dir, &channel.typ);

            let amount = process(&base, target_mip_index, "avg", |a, b| (a + b) / 2.0);
            progress.advance(amount, &mut progress_callback);

            process(&base, target_mip_index, "min", |a, b| a.min(b));
            progress.advance(amount, &mut progress_callback);

            process(&base, target_mip_index, "max", |a, b| a.max(b));
            progress.advance(amount, &mut progress_callback);
        }
    }
}

fn file_name_base(directory: &str, channel: &DataType) -> String {
    format!("{}/{:?}-rate", directory, channel)
}

fn file_name(base: &str, mip_index: i32, filter_name: &str) -> String {
    if mip_index == 0 {
        format!("{}-0.bin", base)
    } else {
        format!("{}-{}-{}.bin", base, mip_index, filter_name)
    }
}

fn process(
    filename_base: &str,
    target_mip_index: i32,
    filter_name: &str,
    filter_impl: impl Fn(f32, f32) -> f32,
) -> u64 {
    let path = file_name(&filename_base, target_mip_index - 1, filter_name);
    let input = File::open(path).unwrap();
    let input_bytes = input.metadata().unwrap().len();
    let mut input = BufReader::new(input);

    let path = file_name(&filename_base, target_mip_index, filter_name);
    let output = File::create(path).unwrap();
    let mut output = BufWriter::new(output);

    let mut buffer = [0; 8];
    while let Ok(()) = input.read_exact(&mut buffer) {
        let a = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let b = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let mipped = filter_impl(a, b);
        output.write_all(&mipped.to_le_bytes()).unwrap();
    }

    input_bytes
}
