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

    let (mut outputs, sample_rate_multipliers) =
        setup::get_output_info(&descriptor, output_dir);
    let frame_sequence_size = descriptor.frame_sequence_size();

    let read_sequence = setup::build_read_sequence(&descriptor, &outputs);

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
        progress_tracker.advance(frame_sequence_size as u64, &mut progress_callback);
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
