use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file = File::open("data/sample.bin").unwrap();
    let mut reader = BufReader::new(file);

    let mut counter: u8 = 0;

    let mut data_frame = vec![0u8; 56];
    let mut bytes_so_far = 0;
    let mut last_notification = 0;
    while let Ok(()) = reader.read_exact(&mut data_frame) {
        for i in 0..56 {
            counter = counter.wrapping_add(data_frame[i]);
        }
        bytes_so_far += 56;
        if bytes_so_far - last_notification > 10_000_000 {
            println!("{}M", bytes_so_far / 1_000_000);
            last_notification = bytes_so_far;
        }
    }

    println!("{}", counter);
}
