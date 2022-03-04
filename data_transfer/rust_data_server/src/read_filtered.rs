use serde::{Deserialize, Serialize};

use crate::{
    read::{self, ReadSamplesParams}, fir_lowpass::fir_lowpass_kernel,
};

const ORDER: usize = 1024;

/// All the parameters necessary to specify what data to read.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadFilteredSamplesParams {
    source: ReadSamplesParams,
    /// 0.1 means cutoff at 10% of the Nyquist frequency (which is half the
    /// sample rate)
    relative_cutoff: f32,
}

fn make_filter(relative_cutoff: f32) -> Vec<f32> {
    fir_lowpass_kernel(ORDER, relative_cutoff)
}

fn convolve(data: &[f32], kernel: &[f32]) -> f32 {
    let mut result = 0.0;
    for (d, k) in data.iter().copied().zip(kernel.iter().copied()) {
        result += d * k;
    }
    result
}

pub fn read_filtered_samples(
    ReadFilteredSamplesParams {
        mut source,
        relative_cutoff,
    }: ReadFilteredSamplesParams,
) -> Result<Vec<f32>, String> {
    if source.start <= (ORDER / 2) as u64{
        return Err(format!(
            "Start for filtered data must be at least {} samples.",
            ORDER / 2
        ));
    }
    let old_range = source.end - source.start;
    source.start -= (ORDER / 2) as u64;
    source.end += (ORDER / 2) as u64;
    let source_data = read::read_samples(source)?;
    let mut output = Vec::new();
    let kernel = make_filter(relative_cutoff);
    for i in 0..(old_range as usize) {
        output.push(convolve(&source_data[i..i+ORDER], &kernel));
    }
    Ok(output)
}
