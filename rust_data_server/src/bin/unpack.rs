use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{prelude::*, BufReader, BufWriter},
};

use rust_data_server::{
    data_format::{DataType, PackedFileDescriptor},
    hamming,
};

type Outputs = Vec<(DataType, BufWriter<File>)>;
type ReadSequence<'a> = Vec<ReadStep<'a>>;
type SampleRateInfo = HashMap<DataType, u8>;

struct ReadStep<'a> {
    num_bytes: usize,
    unpacker: &'a DataType,
    /// Indexes into an instance of Outputs, says which output each unpacked
    /// value should be written to.
    destinations: Vec<usize>,
}

struct ProgressTracker {
    total_bytes: u64,
    bytes_so_far: u64,
    last_notification: u64,
}

impl ProgressTracker {
    pub fn advance(&mut self, amount: u64) {
        self.bytes_so_far += amount;
        if self.bytes_so_far - self.last_notification > 10_000_000 {
            println!(
                "{}M/{}M",
                self.bytes_so_far / 1_000_000,
                self.total_bytes / 1_000_000
            );
            self.last_notification = self.bytes_so_far;
        }
    }
}

fn main() {
    let name = get_name_from_command_line();
    let descriptor = rust_data_server::example_file_descriptor();
    let (mut progress_tracker, mut input) = load_input_file(&name);
    let output_dir = create_output_dir(&name);

    let (mut outputs, sample_rate_multipliers) = get_output_info(&descriptor, output_dir);
    let frame_sequence_size = descriptor.frame_sequence_size();

    let read_sequence = build_read_sequence(&descriptor, &outputs);

    let mut data_frame = vec![0u8; frame_sequence_size];
    while let Ok(()) = input.read_exact(&mut data_frame) {
        if let Err(_) = hamming::decode(&mut data_frame) {
            panic!("Data is corrupt.");
        }
        let mut frame_ptr = &data_frame[..];
        for step in &read_sequence {
            execute_read_step(step, frame_ptr, &mut outputs);
            frame_ptr = &frame_ptr[step.num_bytes..];
        }
        progress_tracker.advance(frame_sequence_size as u64);
    }
}

fn execute_read_step(step: &ReadStep, frame_ptr: &[u8], outputs: &mut Outputs) {
    let values = step
        .unpacker
        .unpack(&frame_ptr[..step.num_bytes])
        .expect("Data is corrupt");
    for (&destination, value) in step.destinations.iter().zip(values.into_iter()) {
        outputs[destination]
            .1
            .write_all(&value.to_le_bytes())
            .unwrap();
    }
}

fn build_read_sequence<'a>(
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

fn get_output_info(
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

fn create_output_dir(name: &String) -> String {
    let output_dir = format!("cache/{}", name);
    fs::create_dir_all(&output_dir).unwrap();
    output_dir
}

fn load_input_file(name: &String) -> (ProgressTracker, BufReader<File>) {
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
