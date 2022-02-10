use std::{env, fs::File, io::BufReader};

use rust_data_server::ingestion::Phase;

fn main() {
    let name = get_name_from_command_line();
    let (size, input) = open_file(&name);
    rust_data_server::ingestion::ingest(input, size, &name, print_progress)
}

fn print_progress(phase: Phase, bytes_so_far: u64, total_bytes: u64) {
    println!(
        "{:?} {}M/{}M",
        phase,
        bytes_so_far / 1_000_000,
        total_bytes / 1_000_000
    )
}

fn open_file(name: &String) -> (u64, BufReader<File>) {
    let file = File::open(format!("data/{}.bin", name)).unwrap();
    let size = file.metadata().unwrap().len();
    let input = BufReader::new(file);
    (size, input)
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
