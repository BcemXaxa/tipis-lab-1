use realfft::{RealFftPlanner, num_complex::Complex};

fn retain_fraction(input: &mut [Complex<f64>], retain: f64) {
    let n = input.len();
    if n == 0 || retain >= 1.0 {
        return;
    }
    let keep = if retain <= 0.0 {
        0
    } else {
        ((n as f64) * retain).round() as usize
    }
    .clamp(0, n);

    if keep >= n {
        return;
    }

    let mut idx: Vec<usize> = (0..n).collect();
    idx.select_nth_unstable_by(keep, |&a, &b| {
        input[b]
            .norm_sqr()
            .partial_cmp(&input[a].norm_sqr())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for &i in &idx[keep..] {
        input[i] = Complex::new(0.0, 0.0);
    }
}

fn inverse_fft(mut input: Vec<Complex<f64>>) -> Vec<f64> {
    let len = (input.len() - 1) * 2;
    let mut planner = RealFftPlanner::<f64>::new();
    let fft = planner.plan_fft_inverse(len);

    let mut output = vec![0.0f64; len];
    fft.process(&mut input, &mut output).unwrap();

    let scale = 1.0 / len as f64;
    output.iter().map(|x| x * scale).collect()
}