mod generate_mips;
mod read_descriptor;
mod unpack;

use std::{fs::File, io::Read};

pub use read_descriptor::{parse_descriptor, read_descriptor};

use self::unpack::{DataConsumer, DataFrameReader};

/// Takes an input stream and writes unpacked data to the cache/ folder.
pub fn ingest(
    mut input: impl Read,
    size: u64,
    name: &str,
    progress_callback: impl FnMut(u64, u64),
) -> () {
    std::fs::create_dir_all(format!("cache/{}", name)).unwrap();

    let descriptor = read_and_write_descriptor(&mut input, name);

    let reader = unpack::new_file_backed_reader(size, name, &descriptor, progress_callback);

    read_input_into_reader(input, reader);
}

/// Reads a descriptor from an input stream and writes it to
/// cache/{name}/index.json
fn read_and_write_descriptor(
    input: &mut impl Read,
    name: &str,
) -> crate::data_format::FileDescriptor {
    let descriptor = read_descriptor::read_descriptor(input);
    let descriptor_file = File::create(format!("cache/{}/index.json", name)).unwrap();
    serde_json::to_writer(descriptor_file, &descriptor.unpacked()).unwrap();
    descriptor
}

fn read_input_into_reader(
    mut input: impl Read,
    mut reader: DataFrameReader<impl FnMut(u64, u64), impl DataConsumer>,
) {
    let mut buffer = vec![0; 8192];
    while let Ok(amount) = input.read(&mut buffer) {
        // Amount is zero when there is no more data to read.
        if amount == 0 {
            break;
        }
        reader
            .push_bytes(buffer[0..amount].iter().copied())
            .unwrap();
    }
}
