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

    pub fn name(self) -> &'static str {
        match self {
            Filter::Minimum => "min",
            Filter::Maximum => "max",
            Filter::Average => "avg",
        }
    }
}

type FileWriter = BufWriter<File>;

pub struct RootDataConsumer<W: Write = FileWriter> {
    output: W,
    /// Stores every other piece of data. Once a second piece of data comes in,
    /// it is combined with this first one to generate min, max, and avg values.
    odd_datum: Option<f32>,
    next: [FilteredDataConsumer<W>; 3],
}

impl<W: Write> RootDataConsumer<W> {
    pub fn new_file_backed(name: &str, channel: &DataType) -> RootDataConsumer<FileWriter> {
        let min =
            FilteredDataConsumer::<FileWriter>::new_file_backed(name, channel, 1, Filter::Minimum);
        let max =
            FilteredDataConsumer::<FileWriter>::new_file_backed(name, channel, 1, Filter::Maximum);
        let avg =
            FilteredDataConsumer::<FileWriter>::new_file_backed(name, channel, 1, Filter::Average);
        let base = file_name_base(name, channel);
        let name = file_name(&base, 0, "");
        let file = File::create(name).unwrap();
        RootDataConsumer {
            output: BufWriter::new(file),
            odd_datum: None,
            next: [min, max, avg],
        }
    }
}

impl<W: Write> DataConsumer for RootDataConsumer<W> {
    fn consume(&mut self, datum: f32) -> io::Result<()> {
        if let Some(previous) = self.odd_datum.take() {
            for next in &mut self.next {
                let filtered = next.filter.filter(previous, datum);
                next.consume(filtered)?;
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

impl<W: Write> FilteredDataConsumer<W> {
    fn new_file_backed(
        name: &str,
        channel: &DataType,
        lod: i32,
        filter: Filter,
    ) -> FilteredDataConsumer<FileWriter> {
        let next = if lod < 40 {
            Some(Box::new(Self::new_file_backed(
                name,
                channel,
                lod + 1,
                filter,
            )))
        } else {
            None
        };
        let base = file_name_base(name, &channel);
        let name = file_name(&base, lod, "");
        let file = File::create(name).unwrap();
        let output = FileWriter::new(file);
        FilteredDataConsumer {
            output,
            filter,
            odd_datum: None,
            next,
        }
    }
}

impl<W: Write> DataConsumer for FilteredDataConsumer<W> {
    fn consume(&mut self, datum: f32) -> io::Result<()> {
        if let Some(previous) = self.odd_datum.take() {
            if let Some(next) = &mut self.next {
                let filtered = next.filter.filter(previous, datum);
                next.consume(filtered)?;
            }
        } else {
            self.odd_datum = Some(datum);
        }
        self.output.write_all(&datum.to_le_bytes())?;
        Ok(())
    }
}

fn file_name_base(name: &str, channel: &DataType) -> String {
    format!("cache/{}/{:?}-rate", name, channel)
}

fn file_name(base: &str, mip_index: i32, filter_name: &str) -> String {
    if mip_index == 0 {
        format!("{}-0.bin", base)
    } else {
        format!("{}-{}-{}.bin", base, mip_index, filter_name)
    }
}

pub fn data_consumers_for(name: &str, descriptor: &FileDescriptor) -> Vec<impl DataConsumer> {
    descriptor
        .unpacked_channels
        .iter()
        .map(|ch| RootDataConsumer::<FileWriter>::new_file_backed(name, &ch.typ))
        .collect()
}
