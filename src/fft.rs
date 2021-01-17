// Modules

// Uses
use log::{info, error, debug};
use std::f32::consts::PI;
use num_complex::Complex;

/// Computes Fast Fourier Transform (FFT)
///
/// While there is an existing, more-optimized FFT crate that Rust offers, this was completed
/// as a sample exercise.
/// Existing crate: https://docs.rs/rustfft/5.0.1/rustfft/
///
/// * `signal` - Complex signal
pub fn fft(signal: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    info!("Entered FFT");

    // Split signal into real and imaginary portions
    let mut real: Vec<f32> = vec![0.0; signal.len()];
    let mut imag: Vec<f32> = vec![0.0; signal.len()];

    for i in 0..signal.len() {
        real[i] = signal[i].re;
        imag[i] = signal[i].im;
    }

    scramble(&mut real, &mut imag);
    unscramble(&real, &imag);

    let freq: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); signal.len()];
    return freq;
}

/// Scramble data
///
/// * `x` - Vector of real values
/// * `y` - Vector of imaginary values
fn scramble(x: &mut Vec<f32>, y: &mut Vec<f32>) {
    debug!("FFT: Scrambling");
    if x.len() != y.len() {
        error!("Real and imaginary vectors must be the same size.");
    }
    let n = x.len();
    const BASE: f32 = 2.0;
    let m = ((n as f32).ln() / BASE.ln()) as i32;
    let mut n2: usize = n;
    for _k in 1..m {
        let n1 = n2;
        n2 /= 2;
        let mut angle: f32 = 0.0;
        let arg: f32 = 2.0 * PI / (n1 as f32);
        for j in 0..n2 - 1 {
            let c = angle.cos();
            let s = -(angle.sin());
            for i in (j..n - 1).step_by(n1) {
                let idx = i as usize;
                let kk: usize = idx + n2;
                let xt: f32 = x[idx] - x[kk];
                x[idx] += x[kk];
                let yt: f32 = y[idx] - y[kk];
                y[idx] += y[kk];
                x[kk] = xt * c - yt * s;
                y[kk] = yt * c + xt * s;
            }
            angle = ((j + 1) as f32) * arg;
        }
    }
}  // scramble()

/// Unscramble data
///
/// * `x` - Vector of real values
/// * `y` - Vector of imaginary values
fn unscramble(x: &mut Vec<f32>, y: &mut Vec<f32>) {
    // TODO: Implement
    debug!("FFT: Unscrambling");
    if x.len() != y.len() {
        error!("Real and imaginary vectors must be the same size.");
    }
}