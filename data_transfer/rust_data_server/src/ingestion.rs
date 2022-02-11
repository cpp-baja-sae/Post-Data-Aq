mod generate_mips;
mod read_descriptor;
mod unpack;

use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

pub fn ingest(
    mut input: impl Read,
    size: u64,
    name: &str,
    mut progress_callback: impl FnMut(u64, u64),
) -> () {
    let descriptor = read_descriptor::read_descriptor(&mut input);
    let descriptor_file = File::create(format!("cache/{}/index.json", name)).unwrap();
    serde_json::to_writer(descriptor_file, &descriptor.unpacked()).unwrap();
}
