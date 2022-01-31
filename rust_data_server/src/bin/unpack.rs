use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{prelude::*, BufReader, BufWriter},
};

use rust_data_server::{data_format::DataType, hamming};

fn main() {
    let args: Vec<_> = env::args().collect();
    // The first "argument" is the path to the executable.
    if args.len() != 2 {
        panic!(
            "Expected a single argument, got {} instead.",
            args.len() - 1
        );
    }
    let name = &args[1];
    let descriptor = rust_data_server::example_file_descriptor();
    let input = File::open(format!("data/{}.bin", name)).unwrap();
    let total_bytes = input.metadata().unwrap().len();
    let mut input = BufReader::new(input);
    let output_dir = format!("cache/{}", name);
    fs::create_dir_all(&output_dir).unwrap();

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
    let mut outputs: Vec<_> = outputs.into_iter().collect();

    let mut frame_sequence_size = 0;
    for frame in descriptor.frame_sequence() {
        for typ in frame {
            frame_sequence_size += typ.num_packed_bytes();
        }
    }

    let mut read_sequence = Vec::new();
    for frame in descriptor.frame_sequence() {
        for typ in frame {
            read_sequence.push((
                typ.num_packed_bytes(),
                typ,
                typ.unpacked_types()
                    .into_iter()
                    .map(|unpacked_typ| {
                        outputs
                            .iter()
                            .position(|(out_type, _file)| out_type == &unpacked_typ)
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            ));
        }
    }

    let mut data_frame = vec![0u8; frame_sequence_size];
    let mut bytes_so_far = 0;
    let mut last_notification = 0;
    while let Ok(()) = input.read_exact(&mut data_frame) {
        if let Err(_) = hamming::decode(&mut data_frame) {
            panic!("Data is corrupt.");
        }
        let mut frame_ptr = &data_frame[..];
        for (length, unpacker, destinations) in &read_sequence {
            let length = *length;
            let values = unpacker
                .unpack(&frame_ptr[..length])
                .expect("Data is corrupt");
            for (&destination, value) in destinations.iter().zip(values.into_iter()) {
                outputs[destination]
                    .1
                    .write_all(&value.to_le_bytes())
                    .unwrap();
            }
            frame_ptr = &frame_ptr[length..];
        }
        bytes_so_far += frame_sequence_size;
        if bytes_so_far - last_notification > 10_000_000 {
            println!("{}M/{}M", bytes_so_far / 1_000_000, total_bytes / 1_000_000);
            last_notification = bytes_so_far;
        }
    }
}
