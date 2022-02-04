#!/bin/sh

cd rust_data_server

cargo run --bin=generate_data --release -- sample
cargo run --bin=ingest --release -- sample
