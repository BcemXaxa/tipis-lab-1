use crate::generator::{
    Digital, Harmonic, SignalGenerator, SignalProperties, SignalType, WaveProperties,
};
use crate::modulation::{ModulationType, build_modulator};
use realfft::RealFftPlanner;

pub struct Derived {
    pub samples: Vec<[f64; 2]>,
    pub spectrum: Vec<[f64; 2]>,
}

impl Derived {
    pub fn empty() -> Self {
        Self {
            samples: Vec::new(),
            spectrum: Vec::new(),
        }
    }

    pub fn new(
        signal_type: SignalType,
        wave_properties: WaveProperties,
        signal_properties: SignalProperties,
    ) -> Self {
        let samples = match signal_type {
            SignalType::Digital => Digital::new(wave_properties).produce(signal_properties.clone()),
            SignalType::Harmonic => {
                Harmonic::new(wave_properties).produce(signal_properties.clone())
            }
        };
        let spectrum =
            Self::make_spectrum(samples.iter().map(|[_, y]| *y).collect(), signal_properties);
        Self { samples, spectrum }
    }

    /// Builds a modulated signal from a harmonic carrier and a digital
    /// modulating signal.
    pub fn new_modulated(
        carrier: WaveProperties,
        modulating: WaveProperties,
        kind: ModulationType,
        modulation_index: f64,
        signal_properties: SignalProperties,
    ) -> Self {
        let samples = build_modulator(
            kind,
            Harmonic::new(carrier),
            Digital::new(modulating),
            modulation_index,
        )
        .produce(signal_properties.clone());
        let spectrum =
            Self::make_spectrum(samples.iter().map(|[_, y]| *y).collect(), signal_properties);
        Self { samples, spectrum }
    }

    fn make_spectrum(mut input: Vec<f64>, props: SignalProperties) -> Vec<[f64; 2]> {
        if input.is_empty() {
            return Vec::new();
        }
        let n = input.len();
        let nf = n as f64;

        let mut planner = RealFftPlanner::<f64>::new();
        let fft = planner.plan_fft_forward(n);
        let mut output = fft.make_output_vec();
        fft.process(&mut input, &mut output).unwrap();

        let bin_hz = props.sample_rate / nf;
        let last = output.len() - 1;

        output
            .iter()
            .enumerate()
            .map(|(k, z)| {
                let mag = z.norm();
                let amp = if k == 0 || k == last {
                    mag / nf
                } else {
                    2.0 * mag / nf
                };
                [k as f64 * bin_hz, amp]
            })
            .collect()
    }
}
