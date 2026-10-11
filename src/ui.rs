use std::f64::consts::TAU;

use eframe::egui::{self, Color32, Ui, Window};
use egui_plot::{Legend, Line, Plot, PlotPoints, PlotUi};

use crate::{
    App,
    derivation::Derived,
    generator::{SignalProperties, SignalType, WaveProperties},
};

#[derive(Clone, PartialEq)]
pub struct Controls {
    wave_properties: WaveProperties,
    signal_properties: SignalProperties,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            wave_properties: WaveProperties {
                frequency: 4.0,
                amplitude: 1.0,
                phase: 0.0,
            },
            signal_properties: SignalProperties {
                sample_rate: 1000.0,
                duration: 1.0,
            },
        }
    }
}

struct SignalState {
    signal_type: SignalType,
    controls: Controls,
    derived: Derived,
}
impl SignalState {
    fn new(signal_type: SignalType) -> Self {
        let controls = Controls::default();
        let derived = Derived::new(
            signal_type,
            controls.wave_properties.clone(),
            controls.signal_properties.clone(),
        );

        Self {
            signal_type,
            controls,
            derived,
        }
    }
    fn get_controls(&self) -> Controls {
        self.controls.clone()
    }
    fn set_controls(&mut self, controls: Controls) {
        if self.controls != controls {
            self.derived = Derived::new(
                self.signal_type,
                controls.wave_properties.clone(),
                controls.signal_properties.clone(),
            );
            self.controls = controls;
        }
    }
}

pub struct State {
    merge_signal_plots: bool,
    harmonic: SignalState,
    digital: SignalState,
}
impl Default for State {
    fn default() -> Self {
        Self {
            merge_signal_plots: true,
            harmonic: SignalState::new(SignalType::Harmonic),
            digital: SignalState::new(SignalType::Digital),
        }
    }
}

impl App {
    pub fn ui(ui: &mut Ui, state: &mut State) {
        if ui.button("Configure").clicked() {
            Window::new("Configuration").show(ui.ctx(), |ui| {
                ui.label("Hello");
            });
        }
        ui.checkbox(&mut state.merge_signal_plots, "merge");
        ui.collapsing("Signals", |ui| {
            if state.merge_signal_plots {
                let mut new_controls = state.harmonic.get_controls();
                Self::controls_panel(ui, &mut new_controls);
                state.harmonic.set_controls(new_controls.clone());
                state.digital.set_controls(new_controls);

                Self::make_plot("merged").show(ui, |plot_ui| {
                    Self::signal_series(plot_ui, &mut state.harmonic);
                    Self::signal_series(plot_ui, &mut state.digital);
                });
            } else {
                ui.columns(2, |columns| {
                    columns[0].vertical(|ui| {
                        let mut new_controls = state.harmonic.get_controls();
                        Self::controls_panel(ui, &mut new_controls);
                        state.harmonic.set_controls(new_controls);

                        Self::signal_plot(ui, &mut state.harmonic);
                    });
                    columns[1].vertical(|ui| {
                        let mut new_controls = state.digital.get_controls();
                        Self::controls_panel(ui, &mut new_controls);
                        state.digital.set_controls(new_controls);

                        Self::signal_plot(ui, &mut state.digital);
                    });
                });
            }
        });
        ui.collapsing("Spectre", |ui| {
            if state.merge_signal_plots {
                
            }
        });
    }
    fn controls_panel(ui: &mut Ui, controls: &mut Controls) {
        let Controls {
            wave_properties: w,
            signal_properties: s,
        } = controls;

        ui.horizontal(|ui| {
            ui.label("frequency:");
            ui.add(egui::DragValue::new(&mut w.frequency).speed(0.1));

            ui.label("amplitude:");
            ui.add(egui::DragValue::new(&mut w.amplitude).speed(0.1));

            ui.label("phase:");
            let mut phase_tmp = (w.phase * TAU) as f32;
            if ui.drag_angle_tau(&mut phase_tmp).changed() {
                w.phase = (phase_tmp as f64 / TAU).rem_euclid(1.0);
            }
        });
        ui.horizontal(|ui| {
            ui.label("duration:");
            ui.add(egui::DragValue::new(&mut s.duration).speed(0.5));

            ui.label("sample rate:");
            ui.add(egui::DragValue::new(&mut s.sample_rate).speed(2.0));
        });
    }
    fn signal_series(plot_ui: &mut PlotUi, signal: &mut SignalState) {
        let (signal_name, color) = match signal.signal_type {
            SignalType::Digital => ("digital_signal", Color32::YELLOW),
            SignalType::Harmonic => ("harmonic_signal", Color32::RED),
        };
        plot_ui.line(Line::new(signal_name, signal.derived.samples.clone()).color(color));
    }

    fn signal_plot(ui: &mut Ui, state: &mut SignalState) {
        let name = match state.signal_type {
            SignalType::Digital => "digital",
            SignalType::Harmonic => "harmonic",
        };

        Self::make_plot(name).show(ui, |plot_ui| {
            Self::signal_series(plot_ui, state);
        });
    }

    fn make_plot(name: &str) -> Plot<'_> {
        Plot::new(name)
            .legend(Legend::default().title(name))
            .x_axis_label("time, s")
            .y_axis_label("voltage, V")
    }
}
