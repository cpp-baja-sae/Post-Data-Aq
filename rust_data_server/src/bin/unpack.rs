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

    let mut frame_sequence_size = 0;
    for frame in descriptor.frame_sequence() {
        for typ in frame {
            frame_sequence_size += typ.num_packed_bytes();
        }
    }

    let mut data_frame = vec![0u8; frame_sequence_size];
    while let Ok(()) = input.read_exact(&mut data_frame) {
        if let Err(_) = hamming::decode(&mut data_frame) {
            panic!("Data is corrupt.");
        }
        let mut frame_ptr = &data_frame[..];
        let mut progress = 0;
        for frame in descriptor.frame_sequence() {
            for typ in frame {
                progress += 1;
                println!("{}", progress);
                let num_bytes = typ.num_packed_bytes();
                let out_types = typ.unpacked_types();
                let values = typ
                    .unpack(&frame_ptr[..num_bytes])
                    .expect("data is corrupt");
                for (out_typ, value) in out_types.into_iter().zip(values.into_iter()) {
                    outputs
                        .get_mut(&out_typ)
                        .unwrap()
                        .write_all(&value.to_le_bytes())
                        .unwrap();
                }
                frame_ptr = &frame_ptr[num_bytes..];
            }
        }
    }
}
