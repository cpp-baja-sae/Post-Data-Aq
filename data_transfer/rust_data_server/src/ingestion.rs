mod generate_mips;
mod unpack;
mod read_descriptor;

use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Phase {
    Unpacking,
    MipGeneration,
}

pub fn ingest(
    mut input: impl Read,
    size: u64,
    name: &str,
    mut progress_callback: impl FnMut(Phase, u64, u64),
) -> () {
    let descriptor = read_descriptor::read_descriptor(&mut input);
    let descriptor_file = File::create(format!("cache/{}/index.json", name)).unwrap();
    serde_json::to_writer(descriptor_file, &descriptor.unpacked()).unwrap();
    let unpack_pc = |progress, total| progress_callback(Phase::Unpacking, progress, total);
    unpack::unpack(input, size, name, &descriptor, unpack_pc);
    let mips_pc = |progress, total| progress_callback(Phase::MipGeneration, progress, total);
    generate_mips::generate_mips(name, &descriptor, mips_pc);
}
