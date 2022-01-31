use std::io::prelude::*;

use super::{
    setup,
    support_types::{Outputs, ReadStep},
};
use crate::{data_format::PackedFileDescriptor, hamming};

pub fn unpack(
    name: &String,
    descriptor: PackedFileDescriptor,
    mut progress_callback: impl FnMut(u64, u64),
) {
    let (mut progress_tracker, mut input) = setup::load_input_file(&name);
    let output_dir = setup::create_output_dir(&name);
    let (mut outputs, sample_rate_multipliers) = setup::get_output_info(&descriptor, output_dir);
    // How long a single instance of the descriptor's frame sequence is, dictates
    // how much data we try to read at once.
    let frame_sequence_size = descriptor.frame_sequence_size();

    // This defines a series of operations to perform repeatedly to read from the
    // file. It specifies how many bytes to read at a time, what to use to decode
    // those bytes, and where the decoded floats should go.
    let read_sequence = setup::build_read_sequence(&descriptor, &outputs);

    // read_sequence says how to read a single occurence of the entire frame
    // sequence specified in the descriptor.
    let mut data_frame = vec![0u8; frame_sequence_size];
    while let Ok(()) = input.read_exact(&mut data_frame) {
        if let Err(_) = hamming::decode(&mut data_frame) {
            panic!("Data is corrupt.");
        }
        // This is the variable decoders read data from.
        let mut frame_ptr = &data_frame[..];
        for step in &read_sequence {
            execute_read_step(step, frame_ptr, &mut outputs);
            // Advance the pointer for the next decoder.
            frame_ptr = &frame_ptr[step.num_bytes..];
        }
        progress_tracker.advance(frame_sequence_size as u64, &mut progress_callback);
    }
}

fn execute_read_step(step: &ReadStep, frame_ptr: &[u8], outputs: &mut Outputs) {
    // Unpack data into individual values.
    let values = step
        .unpacker
        .unpack(&frame_ptr[..step.num_bytes])
        .expect("Data is corrupt");
    // Use the destinations in the read step to write the unpacked data to files.
    for (&destination, value) in step.destinations.iter().zip(values.into_iter()) {
        outputs[destination]
            .1
            .write_all(&value.to_le_bytes())
            .unwrap();
    }
}
