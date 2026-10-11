use realfft::RealFftPlanner;
use crate::generator::{
    Digital, Harmonic, SignalGenerator, SignalProperties, SignalType, WaveProperties,
};

pub struct Derived {
    pub samples: Vec<[f64; 2]>,
    pub spectre: Vec<f64>,
}

impl Derived {
    pub fn new(
        signal_type: SignalType,
        wave_properties: WaveProperties,
        signal_properties: SignalProperties,
    ) -> Self {
        let samples = match signal_type {
            SignalType::Digital => Digital::new(wave_properties).produce(signal_properties),
            SignalType::Harmonic => Harmonic::new(wave_properties).produce(signal_properties),
        };
        let spectre = samples.iter().map(|[_, y]| *y).collect();
        Self { samples, spectre }
    }

    fn make_spectre(mut input: Vec<f64>) -> Vec<f64> {
        if input.is_empty() {
            return Vec::new();
        }
        let mut planner = RealFftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(input.len());
        let mut output = fft.make_output_vec();
        fft.process(&mut input, &mut output).unwrap();
        output.into_iter().map(|z| z.norm()).collect()
    }
}
