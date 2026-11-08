use std::f64::consts::TAU;

use crate::generator::{Digital, Harmonic, SignalGenerator, SignalProperties};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ModulationType {
    Am,
    Fm,
    Pm,
}

impl ModulationType {
    pub const ALL: [Self; 3] = [Self::Am, Self::Fm, Self::Pm];
}

#[derive(Clone, Copy, PartialEq)]
pub struct ModulationSettings {
    pub am_index: f64,
    pub fm_index: f64,
    pub pm_index: f64,
}

impl Default for ModulationSettings {
    fn default() -> Self {
        Self {
            am_index: 0.7,
            fm_index: 1.0,
            pm_index: 0.25,
        }
    }
}

impl ModulationSettings {
    pub fn index(self, kind: ModulationType) -> f64 {
        match kind {
            ModulationType::Am => self.am_index,
            ModulationType::Fm => self.fm_index,
            ModulationType::Pm => self.pm_index,
        }
    }
}

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

        let compute_sample =
            |phase: f64| -> f64 { (phase * TAU).sin() * self.carrier_wave.amplitude };

        out.push([left, compute_sample(phase_accumulator)]);

        while left < right {
            let modulating_signal = self.modulating_signal.get_at(left);
            let instant_freq =
                self.carrier_wave.frequency * (1.0 + self.modulation_index * modulating_signal);

            phase_accumulator = (phase_accumulator + instant_freq * dt).rem_euclid(1.0);

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

/// Builds a modulator from a harmonic carrier and a digital modulating signal.
pub fn build_modulator(
    kind: ModulationType,
    carrier: Harmonic,
    modulating: Digital,
    modulation_index: f64,
) -> Box<dyn SignalGenerator> {
    match kind {
        ModulationType::Am => Box::new(AmplitudeModulation::new(
            carrier,
            modulating,
            modulation_index,
        )),
        ModulationType::Fm => Box::new(FrequencyModulation::new(
            carrier,
            modulating,
            modulation_index,
        )),
        ModulationType::Pm => Box::new(PhaseModulation::new(carrier, modulating, modulation_index)),
    }
}
