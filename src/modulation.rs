use std::f64::consts::TAU;

use crate::generator::{Digital, Harmonic, SignalGenerator, SignalProperties};

pub struct AmplitudeModulation {
    carrier_wave: Harmonic,
    modulating_signal: Digital,
    modulation_index: f64,
}
impl AmplitudeModulation {
    pub fn new(carrier_wave: Harmonic, modulating_signal: Digital, modulation_index: f64) -> Self {
        Self {
            carrier_wave,
            modulating_signal,
            modulation_index,
        }
    }
}
impl SignalGenerator for AmplitudeModulation {
    fn get_at(&self, t: f64) -> f64 {
        let mut new_wave = self.carrier_wave.clone();
        new_wave.amplitude *= 1.0 + self.modulation_index * self.modulating_signal.get_at(t);
        new_wave.get_at(t)
    }
}

pub struct FrequencyModulation {
    carrier_wave: Harmonic,
    modulating_signal: Digital,
    modulation_index: f64,
}
impl FrequencyModulation {
    pub fn new(carrier_wave: Harmonic, modulating_signal: Digital, modulation_index: f64) -> Self {
        Self {
            carrier_wave,
            modulating_signal,
            modulation_index,
        }
    }
}
impl SignalGenerator for FrequencyModulation {
    fn get_at(&self, _t: f64) -> f64 {
        panic!(
            "FrequencyModulation is stateful and does not support stateless `get_at`. Use `produce` instead."
        );
    }

    fn produce(&self, signal_properties: SignalProperties) -> Vec<[f64; 2]> {
        let SignalProperties {
            sample_rate,
            duration,
        } = signal_properties;

        let mut out = Vec::new();
        let dt = (1.0 / sample_rate).abs();
        if !dt.is_normal() {
            return out;
        }

        let [mut left, right] = [0.0f64.min(duration), 0.0f64.max(duration)];

        let mut phase_accumulator = self.carrier_wave.phase;

        let compute_sample = |phase: f64| -> f64 { phase.cos() * self.carrier_wave.amplitude };

        out.push([left, compute_sample(phase_accumulator)]);

        while left < right {
            let modulating_signal = self.modulating_signal.get_at(left);
            let instant_freq =
                self.carrier_wave.frequency + (self.modulation_index * modulating_signal);

            phase_accumulator = (phase_accumulator + instant_freq * dt).rem_euclid(TAU);

            left = (left + dt).min(right);
            out.push([left, compute_sample(phase_accumulator)]);
        }
        out
    }
}

pub struct PhaseModulation {
    carrier_wave: Harmonic,
    modulating_signal: Digital,
    modulation_index: f64,
}
impl PhaseModulation {
    pub fn new(carrier_wave: Harmonic, modulating_signal: Digital, modulation_index: f64) -> Self {
        Self {
            carrier_wave,
            modulating_signal,
            modulation_index,
        }
    }
}
impl SignalGenerator for PhaseModulation {
    fn get_at(&self, t: f64) -> f64 {
        let mut new_wave = self.carrier_wave.clone();
        new_wave.phase += self.modulation_index * self.modulating_signal.get_at(t);
        new_wave.get_at(t)
    }
}
