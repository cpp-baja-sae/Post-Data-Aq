mod generate_mips;
mod read_descriptor;
mod unpack;

use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use self::unpack::DataFrameReader;

pub fn ingest(
    mut input: impl Read,
    size: u64,
    name: &str,
    progress_callback: impl FnMut(u64, u64),
) -> () {
    let descriptor = read_descriptor::read_descriptor(&mut input);
    let descriptor_file = File::create(format!("cache/{}/index.json", name)).unwrap();
    serde_json::to_writer(descriptor_file, &descriptor.unpacked()).unwrap();
    let mut reader = unpack::new_file_backed(size, name, &descriptor, progress_callback);
    let mut buffer = vec![0; 8192];
    while let Ok(amount) = input.read(&mut buffer) {
        reader.push_bytes(buffer[0..amount].iter().copied()).unwrap();
    }
}
