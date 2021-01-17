// Imported modules
mod fft;

// Uses
use simple_logger::SimpleLogger;
use log::{info};
use std::f32::consts::PI;
use num_complex::Complex;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Testing FFT...");

    const SIGNAL_LENGTH: usize = 64;   // Length of signal

    // Define time vector
    let mut time: Vec<f32> = vec![0.0; SIGNAL_LENGTH];

    // Various functions. Treat as complex signals, even if they're not. It's an interface requirement
    let mut sin_fn: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); time.len()];  // sin(2*pi*t)
    let mut cos_fn: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); time.len()];  // cos(2*pi*t)
    let mut step_fn: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); time.len()];  // Step function

    // Populate vectors
    for i in 0..SIGNAL_LENGTH {
        let f = i as f32;
        let sl = SIGNAL_LENGTH as f32;
        time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
        let radians = time[i] * 2.0 * PI;
        sin_fn[i].re = radians.sin();
        cos_fn[i].re = radians.cos();
        if i > SIGNAL_LENGTH / 2 {
            step_fn[i].re = 1.0;  // Step in latter half
        }
    }

    // Execute FFT
    let mut _freq: Vec<Complex<f32>>;
    _freq = fft::fft(sin_fn);
    _freq = fft::fft(cos_fn);
    _freq = fft::fft(step_fn);
}