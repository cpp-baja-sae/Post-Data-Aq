//! A "mip" is a computer science term meaning a lower resolution version of
//! some existing data. The process of generating mips creates new data that are
//! lower sample rate versions of existing data, for use in building a
//! responsive interface.

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};

fn file_name(base: &str, mip_index: i32, filter_name: &str) -> String {
    if mip_index == 0 {
        format!("{}-0.bin", base)
    } else {
        format!("{}-{}-{}.bin", base, mip_index, filter_name)
    }
}

fn main() {
    let name = get_name_from_command_line();
    let descriptor = rust_data_server::example_file_descriptor().unpacked();
    let mut progress_printer = |_, _| ();

    let data_dir = format!("cache/{}", name);
    let total_files = descriptor.channels.len() * 3;

    for target_mip_index in 1..20 {
        let mut completed_files = 0;
        for channel in &descriptor.channels {
            let base = format!("{}/{:?}-rate", data_dir, channel.typ);

            process(
                &base,
                target_mip_index,
                "avg",
                |a, b| (a + b) / 2.0,
                &mut progress_printer,
            );
            completed_files += 1;
            println!("{}/{}", completed_files, total_files);

            process(
                &base,
                target_mip_index,
                "min",
                |a, b| a.min(b),
                &mut progress_printer,
            );
            completed_files += 1;
            println!("{}/{}", completed_files, total_files);

            process(
                &base,
                target_mip_index,
                "max",
                |a, b| a.max(b),
                &mut progress_printer,
            );
            completed_files += 1;
            println!("{}/{}", completed_files, total_files);
        }
    }
}

fn process(
    filename_base: &str,
    target_mip_index: i32,
    filter_name: &str,
    filter_impl: impl Fn(f32, f32) -> f32,
    progress_callback: &mut impl FnMut(u64, u64),
) {
    let path = file_name(&filename_base, target_mip_index - 1, filter_name);
    let input = File::open(path).unwrap();
    let input_bytes = input.metadata().unwrap().len();
    let mut input = BufReader::new(input);

    let path = file_name(&filename_base, target_mip_index, filter_name);
    let output = File::create(path).unwrap();
    let mut output = BufWriter::new(output);

    let mut bytes_so_far = 0;
    let mut buffer = [0; 8];
    while let Ok(()) = input.read_exact(&mut buffer) {
        let a = f32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let b = f32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
        let mipped = filter_impl(a, b);
        output.write(&mipped.to_le_bytes()).unwrap();
        bytes_so_far += 8;
        if bytes_so_far % 100_000_000 == 0 {
            progress_callback(bytes_so_far, input_bytes);
        }
    }
}

fn get_name_from_command_line() -> String {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "Expected a single argument, got {} instead.",
            args.len() - 1
        );
    }
    args.into_iter().nth(1).unwrap()
}
