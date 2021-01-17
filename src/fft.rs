
use num_complex::Complex;

/// Computes Fast Fourier Transform (FFT)
///
/// While there is an existing, more-optimized FFT crate that Rust offers, this was completed
/// as a sample exercise.
/// Existing crate: https://docs.rs/rustfft/5.0.1/rustfft/
///
/// * `signal` - Complex signal
pub fn fft(signal: Vec<Complex<f32>>) -> Vec<Complex<f32>> {

    // Split signal into real and imaginary portions
    let mut real: Vec<f32> = vec![0.0; signal.len()];
    let mut imag: Vec<f32> = vec![0.0; signal.len()];

    for i in 0..signal.len() {
        real[i] = signal[i].re;
        imag[i] = signal[i].im;
    }

    scramble(real, imag);
    unscramble(real, imag);

    let freq: Vec<f32> = vec![0.0; signal.len()];
    return freq;
}

/// Scramble data
fn scramble(real: Vec<f32>, imag: Vec<f32>) {
    // TODO: Implement
}

/// Unscramble data
fn unscramble(real: Vec<f32>, imag: Vec<f32>) {
    // TODO: Implement
}