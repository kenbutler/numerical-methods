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
/// * `sampling_frequency` - Frequency at which signal is sampled
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
    for i in 0..magnitudes.len() {
        magnitudes[i] = (real[i] * real[i] + imag[i] * imag[i]).sqrt();
    }

    // Set frequencies
    let mut freq: Vec<f64> = vec![0.0; signal.len() / 2];
    let stop = signal.len() * (sampling_frequency as usize / 2 - 1);
    let mut idx = 0 as usize;
    for i in (0..stop).step_by(sampling_frequency as usize) {
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
    const BASE: f32 = 2.0;
    let m = ((n as f32).ln() / BASE.ln()) as i32;
    let mut n2: usize = n;
    for _k in 0..m {
        let n1 = n2;
        n2 /= 2;
        let mut angle: f64 = 0.0;
        let arg = 2.0 * PI / (n1 as f64);
        for j in 0..n2 {
            let c = angle.cos();
            let s = -(angle.sin());
            for i in (j..n - 1).step_by(n1) {
                let idx = i as usize;
                let kk: usize = idx + n2;
                let xt: f64 = x[idx] - x[kk];
                let yt: f64 = y[idx] - y[kk];
                x[idx] += x[kk];
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

    for i in 0..n - 2 {
        if i < j {
            // TODO: Is there a better way to swap values here?
            let idx_i = i as usize;
            let idx_j = j as usize;
            let xt = x[idx_j];
            let yt = y[idx_j];
            x[idx_j] = x[idx_i];
            x[idx_i] = xt;
            y[idx_j] = y[idx_i];
            y[idx_i] = yt;
        }
        k = n / 2;
        while k <= j {
            j -= k;
            k /= 2;
        }
        j += k;
    }
    for i in 0..n - 1 {
        let idx_i = i as usize;
        x[idx_i] /= n as f64;
        y[idx_i] /= n as f64;
    }
}  // unscramble()

/// Unit tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_step() {
        info!("FFT of Step Function");
        // Set up
        const SIGNAL_LENGTH: usize = 64;   // Length of signal
        let mut time: Vec<f64> = vec![0.0; SIGNAL_LENGTH];
        let mut step_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // Step function
        for i in 0..SIGNAL_LENGTH {
            let f = i as f64;
            let sl = SIGNAL_LENGTH as f64;
            time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
            step_fn[i].re = 1.0;
        }
        let sampling_freq: f64 = time.len() as f64 / time[time.len() - 1];
        // Execute
        let res = fft(step_fn, sampling_freq);
        let expected_freq = 0.0;
        for i in 0..res.0.len() {
            if i == expected_freq as usize {
                assert_eq!(res.0[i].round(), expected_freq);  // Frequency
                assert_eq!(res.1[i], 1.0);  // Magnitude
            } else {
                assert_eq!(res.1[i].round(), 0.0);  // Magnitude
            }
        }
    }

    #[test]
    fn test_sine() {
        info!("FFT of Sine Function");
        // Set up
        const SIGNAL_LENGTH: usize = 64;   // Length of signal
        let mut time: Vec<f64> = vec![0.0; SIGNAL_LENGTH];
        let mut sin_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // sin(2*pi*t)
        for i in 0..SIGNAL_LENGTH {
            let f = i as f64;
            let sl = SIGNAL_LENGTH as f64;
            time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
            let radians = 4.0 * PI * time[i];
            sin_fn[i].re = radians.sin();
        }
        let sampling_freq: f64 = time.len() as f64 / time[time.len() - 1];
        // Execute
        let res = fft(sin_fn, sampling_freq);
        let expected_freq = 2.0;
        for i in 0..res.0.len() {
            if i == expected_freq as usize {
                assert_eq!(res.0[i].round(), expected_freq);  // Frequency
                assert_eq!(res.1[i], 0.5);  // Magnitude
            } else {
                assert_eq!(res.1[i].round(), 0.0);  // Magnitude
            }
        }
    }

    #[test]
    fn test_cosine() {
        info!("FFT of Cosine Function");
        // Set up
        const SIGNAL_LENGTH: usize = 64;   // Length of signal
        let mut time: Vec<f64> = vec![0.0; SIGNAL_LENGTH];
        let mut cos_fn: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); time.len()];  // sin(2*pi*t)
        for i in 0..SIGNAL_LENGTH {
            let f = i as f64;
            let sl = SIGNAL_LENGTH as f64;
            time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
            let radians = 2.0 * PI * time[i];
            cos_fn[i].re = radians.cos();
        }
        let sampling_freq: f64 = time.len() as f64 / time[time.len() - 1];
        // Execute
        let res = fft(cos_fn, sampling_freq);
        for i in 0..res.0.len() {
            if i == 1 {
                assert_eq!(res.0[i].round(), 1.0);  // Frequency
                assert_eq!(res.1[i], 0.5);  // Magnitude
            } else {
                assert_eq!(res.1[i].round(), 0.0);  // Magnitude
            }
        }
    }
}