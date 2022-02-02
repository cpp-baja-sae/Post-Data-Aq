use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

use crate::data_format::{UnpackedChannelDescriptor, UnpackedFileDescriptor};

pub fn list_datasets() -> Vec<String> {
    let dir = fs::read_dir("cache").unwrap();
    dir.map(|entry| {
        let entry = entry.unwrap();
        entry.file_name().to_string_lossy().to_string()
    })
    .collect()
}

pub fn dataset_descriptor(name: &str) -> Option<UnpackedFileDescriptor> {
    let file = File::open(format!("cache/{}/index.json", name)).ok()?;
    Some(serde_json::from_reader(file).unwrap())
}

pub fn read_samples(
    name: &str,
    channel: UnpackedChannelDescriptor,
    rate_modifier: u8,
    filter: &str,
    start: u64,
    end: u64,
) -> Result<Vec<f32>, String> {
    let file_path = format!("cache/{}/{:?}-rate-{}", name, channel, rate_modifier);
    if filter != "min" && filter != "max" && filter != "avg" {
        return Err(format!(
            "Expected filter to be min, max, or avg, got {} instead",
            filter
        ));
    }
    let file_path = if rate_modifier == 0 {
        format!("{}.bin", file_path)
    } else {
        format!("{}-{}.bin", file_path, filter)
    };
    let mut file = File::open(file_path)
        .map_err(|_| format!("The specified name, channel, or rate modifier is invalid."))?;
    file.seek(SeekFrom::Start(start)).unwrap();
    let mut reader = BufReader::new(file);
    let mut result = Vec::new();
    let mut bytes = [0; 4];
    let length = (end - start) as usize;
    while let Ok(()) = reader.read_exact(&mut bytes) {
        result.push(f32::from_le_bytes(bytes));
        if result.len() >= length {
            break;
        }
    }
    Ok(result)
}
