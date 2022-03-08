use std::{f32::consts::PI, ops::Rem};

fn sinc(x: f32) -> f32 {
    if x.abs() < 1e-3 {
        // The limit that the actual formula approaches at 0.
        1.0
    } else {
        let x = x * PI;
        x.sin() / x
    }
}

/// https://en.wikipedia.org/wiki/Window_function#Welch_window
fn welch_window(index: usize, size: usize) -> f32 {
    let (index, size) = (index as f32, size as f32);
    1.0 / (index - (size / 2.0)).powi(2) / (size / 2.0)
}

/// Windowed Sinc function.
/// https://www.analog.com/media/en/technical-documentation/dsp-book/dsp_book_Ch16.pdf
pub fn fir_lowpass_kernel(size: usize, cutoff: f32) -> Vec<f32> {
    let half_size = (size as f32 - 1.0) / 2.0;

    todo!()
}

#[test]
fn test_fir_lowpass_kernel() {
    let actual = fir_lowpass_kernel(32, 0.5);
}
