// Imported modules
mod fft;

// Uses
use simple_logger::SimpleLogger;
use log::{info, debug};
use std::f64::consts::PI;
use num_complex::Complex;

fn main() {
    SimpleLogger::new().init().unwrap();

    info!("Testing FFT...");

    const SIGNAL_LENGTH: usize = 64;   // Length of signal

    // Define time vector
    let mut time: Vec<f64> = vec![0.0; SIGNAL_LENGTH];

    // Various functions. Treat as complex signals, even if they're not. It's an interface requirement
    let mut sin_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // sin(2*pi*t)
    let mut cos_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // cos(2*pi*t)
    let mut step_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // Step function

    // Populate vectors
    for i in 0..SIGNAL_LENGTH {
        let f = i as f64;
        let sl = SIGNAL_LENGTH as f64;
        time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
        let radians = time[i] * 2.0 * PI;
        sin_fn[i].re = radians.sin();
        cos_fn[i].re = radians.cos();
        if i > SIGNAL_LENGTH / 2 {
            step_fn[i].re = 1.0;  // Step in latter half
        }
    }

    // Execute FFT
    let sampling_freq: f64 = time.len() as f64 / time[time.len() - 1];
    let mut res = fft::fft(sin_fn, sampling_freq);
    for i in 0..res.0.len() {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }
    res = fft::fft(cos_fn, sampling_freq);
    for i in 0..res.0.len() {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }
    res = fft::fft(step_fn, sampling_freq);
    for i in 0..res.0.len() {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }
}