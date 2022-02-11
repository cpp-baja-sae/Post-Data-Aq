use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write},
};

use super::unpack::DataConsumer;
use crate::{
    data_format::{DataType, FileDescriptor},
    util::ProgressTracker,
};

#[derive(Clone, Copy, Debug)]
pub enum Filter {
    Minimum,
    Maximum,
    Average,
}

impl Filter {
    pub fn filter(self, a: f32, b: f32) -> f32 {
        match self {
            Filter::Minimum => a.min(b),
            Filter::Maximum => a.max(b),
            Filter::Average => (a + b) / 2.0,
        }
    }
}

pub struct RootDataConsumer<W: Write> {
    output: W,
    /// Stores every other piece of data. Once a second piece of data comes in,
    /// it is combined with this first one to generate min, max, and avg values.
    odd_datum: Option<f32>,
    next: [FilteredDataConsumer<W>; 3],
}

impl<W: Write> DataConsumer for RootDataConsumer<W> {
    fn consume(&mut self, datum: f32) -> io::Result<()> {
        if let Some(previous) = self.odd_datum.take() {
            for next in &mut self.next {
                let filtered = next.filter.filter(previous, datum);
                next.consume(filtered);
            }
        } else {
            self.odd_datum = Some(datum);
        }
        self.output.write_all(&datum.to_le_bytes())?;
        Ok(())
    }
}

pub struct FilteredDataConsumer<W: Write> {
    output: W,
    filter: Filter,
    /// Stores every other piece of data. Once a second piece of data comes in,
    /// it is combined with this first one to generate min, max, and avg values.
    odd_datum: Option<f32>,
    next: Option<Box<Self>>,
}

impl<W: Write> DataConsumer for FilteredDataConsumer<W> {
    fn consume(&mut self, datum: f32) -> io::Result<()> {
        if let Some(previous) = self.odd_datum.take() {
            if let Some(next) = &mut self.next {
                let filtered = next.filter.filter(previous, datum);
                next.consume(filtered);
            }
        } else {
            self.odd_datum = Some(datum);
        }
        self.output.write_all(&datum.to_le_bytes())?;
        Ok(())
    }
}

fn file_name_base(directory: &str, channel: &DataType) -> String {
    format!("{}/{:?}-rate", directory, channel)
}

fn file_name(base: &str, mip_index: i32, filter_name: &str) -> String {
    if mip_index == 0 {
        format!("{}-0.bin", base)
    } else {
        format!("{}-{}-{}.bin", base, mip_index, filter_name)
    }
}
