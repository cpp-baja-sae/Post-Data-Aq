use std::{f32::consts::PI, ops::Rem};

fn sinc(x: f32) -> f32 {
    if x.abs() < 1e-3 {
        // The limit that the actual formula approaches at 0.
        1.0
    } else {
        let x = x * PI;
        let res = x.sin() / x;
        if res.is_infinite() {
            panic!("{:?}", x)
        }
        res
    }
}

/// https://en.wikipedia.org/wiki/Window_function#Welch_window
fn welch_window(index: usize, size: usize) -> f32 {
    let (index, size) = (index as f32, size as f32);
    let res = 1.0 - ((index - size / 2.0) / (size / 2.0)).powi(2);
    if res.is_infinite() {
        panic!("{:?} {:?}", index, size);
    }
    res
}

/// Windowed Sinc function.
/// https://www.analog.com/media/en/technical-documentation/dsp-book/dsp_book_Ch16.pdf
pub fn fir_lowpass_kernel(size: usize, cutoff: f32) -> Vec<f32> {
    let half_size = (size as f32 - 1.0) / 2.0;

    let kernel: Vec<_> = (0..size)
        .map(|x| welch_window(x, size) * sinc((x as f32 - half_size) / half_size * cutoff))
        .collect();
    let norm = kernel.iter().copied().sum::<f32>();
    kernel.into_iter().map(|x| x / norm).collect()
}
