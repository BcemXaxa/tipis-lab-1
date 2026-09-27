use std::f64::consts::PI;

pub trait SignalGenerator {
    fn get_at(&self, t: f64) -> f64;
}

pub struct Harmonic {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
}
impl Harmonic {
    pub fn new(freq: f64, amp: f64, phase: f64) -> Self {
        Self {
            frequency: freq,
            amplitude: amp,
            phase: phase,
        }
    }
}

impl SignalGenerator for Harmonic {
    fn get_at(&self, t: f64) -> f64 {
        self.amplitude * ((t * self.frequency + self.phase) * 2.0 * PI).sin()
    }
}

pub struct Digital {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
}
impl Digital {
    pub fn new(freq: f64, amp: f64, phase: f64) -> Self {
        Self {
            frequency: freq,
            amplitude: amp,
            phase: phase,
        }
    }
}
impl SignalGenerator for Digital {
    fn get_at(&self, t: f64) -> f64 {
        ((t * self.frequency + self.phase).rem_euclid(1.0)).round() * self.amplitude
    }
}
