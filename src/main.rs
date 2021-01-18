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
        sin_fn[i].re = (4.0 * PI * time[i]).sin();
        cos_fn[i].re = (2.0 * PI * time[i]).cos();
        step_fn[i].re = 1.0;
    }

    // Execute FFT
    let sampling_freq: f64 = time.len() as f64 / time[time.len() - 1];
    let mut res;

    const NUM_FREQ_2_SHOW: usize = 5;

    info!("FFT of Sine Function");
    res = fft::fft(sin_fn, sampling_freq);
    for i in 0..NUM_FREQ_2_SHOW {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }

    info!("FFT of Cosine Function");
    res = fft::fft(cos_fn, sampling_freq);
    for i in 0..NUM_FREQ_2_SHOW {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }

    info!("FFT of Step Function");
    res = fft::fft(step_fn, sampling_freq);
    for i in 0..NUM_FREQ_2_SHOW {
        debug!("freq = {}, mag = {}", res.0[i], res.1[i]);
    }
}