use std::f64::consts::TAU;

use eframe::egui::{self, Ui};
use egui_plot::{Legend, Line, Plot, PlotPoints};

use crate::{
    App,
    generator::{Digital, Harmonic, SignalGenerator},
};

pub struct Controls {
    frequency: f64,
    amplitude: f64,
    phase: f64,

    sample_rate: f64,
    duration: f64,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            frequency: 5.0,
            amplitude: 1.0,
            phase: 0.0,
            sample_rate: 100.0,
            duration: 1.0,
        }
    }
}

#[derive(Default)]
struct SignalState {
    controls: Controls,
    samples: Vec<f64>,
}

#[derive(Default)]
pub struct State {
    harmonic: SignalState,
    digital: SignalState,
}

impl State {
    fn update() {
        self.harmonic.points.clear();
        self.digital.points.clear();

        let dt = 1.0 / sample_rate;
        if !dt.is_normal() {
            return;
        }

        let harmonic_generator = Harmonic::new(frequency, amplitude, phase);
        let digital_generator = Digital::new(frequency, amplitude, phase);
        let mut push_op = |t| {
            self.harmonic.points.push([t, harmonic_generator.get_at(t)]);
            self.digital.points.push([t, digital_generator.get_at(t)]);
        };

        let [mut left, right] = [0.0f64.min(duration), 0.0f64.max(duration)];
        push_op(left);
        while left < right {
            left = (left + dt).min(right);
            push_op(left);
        }
    }
}

impl App {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut changed = false;

            ui.label("frequency:");
            changed |= ui.add(egui::DragValue::new(frequency).speed(0.1)).changed();

            ui.label("amplitude:");
            changed |= ui.add(egui::DragValue::new(amplitude).speed(0.1)).changed();

            ui.label("phase:");
            let mut phase_temp = (*phase * TAU) as f32;
            if ui.drag_angle_tau(&mut phase_temp).changed() {
                *phase = (phase_temp as f64 / TAU).rem_euclid(1.0);
                changed |= true;
            }

            ui.label("sample rate:");
            changed |= ui
                .add(egui::DragValue::new(sample_rate).speed(2.0))
                .changed();

            ui.label("duration:");
            changed |= ui.add(egui::DragValue::new(duration).speed(0.5)).changed();
        });

        Plot::new("signal_plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(
                    "harmonic_signal",
                    PlotPoints::from(self.state.harmonic.points.clone()),
                ));
                plot_ui.line(Line::new(
                    "digital_signal",
                    PlotPoints::from(self.state.digital.points.clone()),
                ));
            });
    }
}
