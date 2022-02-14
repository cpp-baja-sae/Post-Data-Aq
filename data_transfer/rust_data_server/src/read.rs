use std::{
    fs::{self, File},
    io::{BufReader, Read, Seek, SeekFrom},
};

use serde::{Deserialize, Serialize};

use crate::data_format::{DataType, UnpackedFileDescriptor};

/// Lists all datasets stored in `cache/`
pub fn list_datasets() -> Vec<String> {
    let dir = fs::read_dir("cache").unwrap();
    dir.map(|entry| {
        let entry = entry.unwrap();
        entry.file_name().to_string_lossy().to_string()
    })
    .collect()
}

/// Gets the descriptor for a dataset by reading `cache/{name}/index.json`.
pub fn dataset_descriptor(name: &str) -> Option<UnpackedFileDescriptor> {
    let file = File::open(format!("cache/{}/index.json", name)).ok()?;
    Some(serde_json::from_reader(file).unwrap())
}

/// All the parameters necessary to specify what data to read.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadSamplesParams {
    /// The name of the dataset being read from.
    name: String,
    /// The channel in the dataset being read from.
    channel: DataType,
    /// The sample rate will be divided by 2 to the power of this number. For
    /// example, to get 16x downsampling, use `4`.
    rate_modifier: u8,
    /// The filter to use when downsampling. Has no effect when rate_modifier is
    /// zero.
    filter: String,
    /// The index to start reading at.
    start: u64,
    /// The index to stop reading before (it is exclusive.)
    end: u64,
}

pub fn read_samples(
    ReadSamplesParams {
        name,
        channel,
        rate_modifier,
        filter,
        start,
        end,
    }: ReadSamplesParams,
) -> Result<Vec<f32>, String> {
    let file = open_source_file(name, channel, rate_modifier, filter)?;
    let result = read_from_stream(file, start, end);
    Ok(result)
}

fn open_source_file(
    name: String,
    channel: DataType,
    rate_modifier: u8,
    filter: String,
) -> Result<File, String> {
    let file_path = make_file_path(name, channel, rate_modifier, filter)?;
    let file = File::open(file_path)
        .map_err(|_| format!("The specified name, channel, or rate modifier is invalid."))?;
    Ok(file)
}

fn read_from_stream(file: File, start: u64, end: u64) -> Vec<f32> {
    let (mut reader, length) = setup_reader(file, start, end);

    let mut result = Vec::new();
    let mut bytes = [0; 4];
    while let Ok(()) = reader.read_exact(&mut bytes) {
        result.push(f32::from_le_bytes(bytes));
        if result.len() >= length {
            break;
        }
    }

    result
}

fn setup_reader(mut file: File, start: u64, end: u64) -> (BufReader<File>, usize) {
    file.seek(SeekFrom::Start(start)).unwrap();
    let reader = BufReader::new(file);
    let length = (end - start) as usize;
    (reader, length)
}

fn make_file_path(
    name: String,
    channel: DataType,
    rate_modifier: u8,
    filter: String,
) -> Result<String, String> {
    validate_filter(&filter)?;
    let file_path = format!("cache/{}/{:?}-rate-{}", name, channel, rate_modifier);
    let file_path = if rate_modifier == 0 {
        format!("{}.bin", file_path)
    } else {
        format!("{}-{}.bin", file_path, filter)
    };
    Ok(file_path)
}

fn validate_filter(filter: &String) -> Result<(), String> {
    if *filter != "min" && *filter != "max" && *filter != "avg" {
        Err(format!(
            "Expected filter to be min, max, or avg, got {} instead",
            filter
        ))
    } else {
        Ok(())
    }
}
