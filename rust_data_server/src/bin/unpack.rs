use std::env;

fn main() {
    let name = get_name_from_command_line();
    let descriptor = rust_data_server::example_file_descriptor();
    let progress_printer = |bytes_so_far, total_bytes| {
        println!("{}M/{}M", bytes_so_far / 1_000_000, total_bytes / 1_000_000)
    };
    rust_data_server::ingestion::unpack(&name, descriptor, progress_printer)
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
