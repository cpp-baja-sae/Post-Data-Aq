use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, BufWriter},
};

use super::support_types::{Outputs, ProgressTracker, ReadSequence, ReadStep, SampleRateInfo};
use crate::data_format::{DataType, PackedFileDescriptor};

pub(super) fn build_read_sequence<'a>(
    descriptor: &'a PackedFileDescriptor,
    outputs: &Outputs,
) -> ReadSequence<'a> {
    let mut read_sequence = Vec::new();
    for frame in descriptor.frame_sequence() {
        for typ in frame {
            read_sequence.push(ReadStep {
                num_bytes: typ.num_packed_bytes(),
                unpacker: typ,
                destinations: find_outputs(outputs, typ),
            });
        }
    }
    read_sequence
}

fn find_outputs(outputs: &Outputs, typ: &DataType) -> Vec<usize> {
    typ.unpacked_types()
        .into_iter()
        .map(|unpacked_type| find_output_with_type(outputs, &unpacked_type))
        .collect::<Vec<_>>()
}

fn find_output_with_type(outputs: &Outputs, with_type: &DataType) -> usize {
    outputs
        .iter()
        .position(|(out_type, _file)| out_type == with_type)
        .unwrap()
}

pub(super) fn get_output_info(
    descriptor: &PackedFileDescriptor,
    output_dir: String,
) -> (Outputs, SampleRateInfo) {
    let mut outputs = HashMap::<DataType, BufWriter<File>>::new();
    let mut sample_rate_multipliers = HashMap::<DataType, u8>::new();
    for frame in descriptor.frame_sequence() {
        for typ in frame {
            for typ in typ.unpacked_types() {
                if outputs.contains_key(&typ) {
                    *sample_rate_multipliers.get_mut(&typ).unwrap() += 1;
                } else {
                    let filename = format!("{}/{:?}-rate-0.bin", &output_dir, typ);
                    let writer = BufWriter::new(File::create(filename).unwrap());
                    outputs.insert(typ.clone(), writer);
                    sample_rate_multipliers.insert(typ, 1);
                }
            }
        }
    }
    let outputs: Vec<_> = outputs.into_iter().collect();
    (outputs, sample_rate_multipliers)
}

pub(super) fn create_output_dir(name: &String) -> String {
    let output_dir = format!("cache/{}", name);
    fs::create_dir_all(&output_dir).unwrap();
    output_dir
}

pub(super) fn load_input_file(name: &String) -> (ProgressTracker, BufReader<File>) {
    let input = File::open(format!("data/{}.bin", name)).unwrap();
    let total_bytes = input.metadata().unwrap().len();
    let input = BufReader::new(input);
    let progress = ProgressTracker {
        total_bytes,
        bytes_so_far: 0,
        last_notification: 0,
    };
    (progress, input)
}
