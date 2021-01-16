
/// Computes Fast Fourier Transform (FFT)
///
/// While there is an existing, more-optimized FFT crate that Rust offers, this was completed
/// as a sample exercise.
/// Existing crate: https://docs.rs/rustfft/5.0.1/rustfft/
///
/// * `signal` - Integer representing the length of the provided signal. Ideally this is a multiple of 2.
pub fn fft(signal: Vec<f32>) {

    // TODO: Determine length of signal
    // TODO: Split signal into real and imaginary portions

    // debug
    for v in signal {
        println!("val={}", v);
    }
    scramble();
    unscramble();
}

fn scramble() {
    // TODO: Implement
}

fn unscramble() {
    // TODO: Implement
}