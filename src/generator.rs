use std::{
    f64::consts::TAU,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy)]
pub enum SignalType {
    Digital,
    Harmonic,
}

#[derive(Clone, Copy, PartialEq)]
pub struct WaveProperties {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
}

#[derive(Clone, PartialEq)]
pub struct SignalProperties {
    pub sample_rate: f64,
    pub duration: f64,
}

pub trait SignalGenerator {
    fn get_at(&self, t: f64) -> f64;
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
        out.push([left, self.get_at(left)]);
        while left < right {
            left = (left + dt).min(right);
            out.push([left, self.get_at(left)]);
        }
        out
    }
}

#[derive(Clone)]
pub struct Harmonic {
    wave_properties: WaveProperties,
}
impl Harmonic {
    pub fn new(wave_properties: WaveProperties) -> Self {
        Self { wave_properties }
    }
}
impl SignalGenerator for Harmonic {
    fn get_at(&self, t: f64) -> f64 {
        let p = &self.wave_properties;
        p.amplitude * ((t * p.frequency + p.phase) * TAU).sin()
    }
}

#[derive(Clone)]
pub struct Digital {
    wave_properties: WaveProperties,
}
impl Digital {
    pub fn new(wave_properties: WaveProperties) -> Self {
        Self {
            wave_properties,
        }
    }
}
impl SignalGenerator for Digital {
    fn get_at(&self, t: f64) -> f64 {
        let p = &self.wave_properties;
        p.amplitude * ((t * p.frequency + p.phase).rem_euclid(1.0)).round()
    }
}

// Convenience implementations

impl DerefMut for Harmonic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wave_properties
    }
}
impl Deref for Harmonic {
    type Target = WaveProperties;
    fn deref(&self) -> &Self::Target {
        &self.wave_properties
    }
}
impl DerefMut for Digital {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wave_properties
    }
}
impl Deref for Digital {
    type Target = WaveProperties;
    fn deref(&self) -> &Self::Target {
        &self.wave_properties
    }
}