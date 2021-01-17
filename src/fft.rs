// Modules

// Uses
use log::{info, error, debug};
use std::f64::consts::PI;
use num_complex::Complex;

/// Computes Fast Fourier Transform (FFT)
///
/// While there is an existing, more-optimized FFT crate that Rust offers, this was completed
/// as a sample exercise.
/// Existing crate: https://docs.rs/rustfft/5.0.1/rustfft/
///
/// * `signal` - Complex signal
pub fn fft(signal: Vec<Complex<f64>>, sampling_frequency: f64) -> (Vec<f64>, Vec<f64>) {
    info!("Entered FFT");

    debug!("Signal length = {}", signal.len());

    // Split signal into real and imaginary portions
    let mut real: Vec<f64> = vec![0.0; signal.len()];
    let mut imag: Vec<f64> = vec![0.0; signal.len()];

    for i in 0..signal.len() {
        real[i] = signal[i].re;
        imag[i] = signal[i].im;
    }

    scramble(&mut real, &mut imag);
    unscramble(&mut real, &mut imag);

    // Set magnitudes
    let mut magnitudes: Vec<f64> = vec![0.0; signal.len()];
    for i in 0..real.len() {
        magnitudes[i] = (real[i] * real[i] + imag[i] * imag[i]).sqrt();
    }

    // Set frequencies
    let mut freq: Vec<f64> = vec![0.0; signal.len() / 2];
    let stop = signal.len() * (sampling_frequency as usize / 2 - 1);
//    let stop: usize = real.len() * ((sampling_frequency / 2.0) as usize - 2);
    let step_size = 2 * ((sampling_frequency / 2.0) as usize) - 1;
    debug!("freq size = {}, stop={}, step size={}", freq.len(), stop, step_size);
    let mut idx = 0 as usize;
    for i in (0..stop).step_by(step_size) {
        freq[idx] = (i as f64) / (real.len() as f64);
        idx += 1;
    }
    return (freq, magnitudes);
}

/// Scramble data
///
/// * `x` - Vector of real values
/// * `y` - Vector of imaginary values
fn scramble(x: &mut Vec<f64>, y: &mut Vec<f64>) {
    debug!("FFT: Scrambling");
    if x.len() != y.len() {
        error!("Real and imaginary vectors must be the same size.");
    }
    let n = x.len();
    const BASE: f64 = 2.0;
    let m = ((n as f64).ln() / BASE.ln()) as i32;
    let mut n2: usize = n;
    for _k in 1..m {
        let n1 = n2;
        n2 /= 2;
        let mut angle: f64 = 0.0;
        let arg: f64 = 2.0 * PI / (n1 as f64);
        for j in 0..n2 - 1 {
            let c = angle.cos();
            let s = -(angle.sin());
            for i in (j..n - 1).step_by(n1) {
                let idx = i as usize;
                let kk: usize = idx + n2;
                let xt: f64 = x[idx] - x[kk];
                x[idx] += x[kk];
                let yt: f64 = y[idx] - y[kk];
                y[idx] += y[kk];
                x[kk] = xt * c - yt * s;
                y[kk] = yt * c + xt * s;
            }
            angle = ((j + 1) as f64) * arg;
        }
    }
}  // scramble()

/// Unscramble data
///
/// * `x` - Vector of real values
/// * `y` - Vector of imaginary values
fn unscramble(x: &mut Vec<f64>, y: &mut Vec<f64>) {
    debug!("FFT: Unscrambling");
    if x.len() != y.len() {
        error!("Real and imaginary vectors must be the same size.");
    }
    let n = x.len() as i32;
    let mut j = 0;
    let mut k: i32;

    for i in 0..n-2 {
        if i < j {
            // TODO: Is there a better way to swap values here?
            let idx_i = i as usize;
            let idx_j = j as usize;
            let xt = x[idx_j];
            x[idx_j] = x[idx_i];
            x[idx_i] = xt;
            let yt = y[idx_j];
            y[idx_j] = y[idx_i];
            y[idx_i] = yt;
        }
        k = n / 2;
        loop {
            if k >= j + 1 {
                break;
            }
            j -= k;
            k /= 2;
        }
        j += k;
    }
    for i in 0..n-1 {
        let idx_i = i as usize;
        x[idx_i] /= n as f64;
        y[idx_i] /= n as f64;
    }
}