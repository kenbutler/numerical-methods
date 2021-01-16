// Modules
mod fft;

 use std::f32::consts::PI;

fn main() {
    println!("Testing FFT...");
    const SIGNAL_LENGTH: usize = 64;   // Length of signal

    // Define time vector
    let mut time: Vec<f32> = vec![0.0; SIGNAL_LENGTH];

    // Various functions
    let mut sin_fn: Vec<f32> = vec![0.0; time.len()];  // sin(2*pi*t)
    let mut cos_fn: Vec<f32> = vec![0.0; time.len()];  // cos(2*pi*t)
    let mut step_fn: Vec<f32> = vec![0.0; time.len()];

    // Populate vectors
    for i in 0..SIGNAL_LENGTH {
        let f = i as f32;
        let sl = SIGNAL_LENGTH as f32;
        time[i] = f / sl;  // Normalization to make trigonometric calculations nicer
        let rad = time[i] * 2.0 * PI;
        sin_fn[i] = rad.sin();
        cos_fn[i] = rad.cos();
        if i > SIGNAL_LENGTH / 2 {
            step_fn[i] = 1.0;
        }
    }

    // Execute FFT
    fft::fft(sin_fn);
}