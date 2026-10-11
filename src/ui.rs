use std::f64::consts::TAU;

use eframe::egui::{self, Color32, Ui, Window};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotUi};
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use taffy::{Style, prelude::*};

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
    harmonic: SignalState,
    digital: SignalState,
    controls_open: bool,
}
impl Default for State {
    fn default() -> Self {
        Self {
            harmonic: SignalState::new(SignalType::Harmonic),
            digital: SignalState::new(SignalType::Digital),
            controls_open: false,
        }
    }
}

// ─── Layout ─────────────────────────────────────────────────────
impl App {
    pub fn ui(ui: &mut Ui, state: &mut State) {
        // ── Toolbar ─────────────────────────────────────────────
        ui.horizontal(|ui| {
            if ui.button("⚙ Configure").clicked() {
                state.controls_open = !state.controls_open;
            }
        });
        ui.separator();

        // ── Floating settings window ────────────────────────────
        if state.controls_open {
            Window::new("Configuration")
                .resizable(true)
                .default_size([520.0, 160.0])
                .show(ui.ctx(), |ui| {
                    ui.columns(2, |cols| {
                        cols[0].heading("Harmonic");
                        let mut c = state.harmonic.get_controls();
                        Self::controls_panel(&mut cols[0], &mut c);
                        state.harmonic.set_controls(c);

                        cols[1].heading("Digital");
                        let mut c = state.digital.get_controls();
                        Self::controls_panel(&mut cols[1], &mut c);
                        state.digital.set_controls(c);
                    });
                });
        }

        // ── 2×2 grid: signals on top, spectrums below ───────────
        use taffy::style_helpers::{fr, length, percent};

        tui(ui, ui.id().with("plot_grid"))
            .reserve_available_space()
            .style(Style {
                display: taffy::Display::Grid,
                grid_template_columns: vec![fr(1.0f32), fr(1.0f32)],
                grid_template_rows: vec![fr(1.0f32), fr(1.0f32)],
                gap: Size {
                    width: length(8.0f32),
                    height: length(8.0f32),
                },
                size: percent(1.0f32),
                align_items: Some(AlignItems::Stretch),
                justify_items: Some(AlignItems::Stretch),
                ..Default::default()
            })
            .show(|tui| {
                tui.ui(|ui| Self::signal_plot(ui, &mut state.harmonic));
                tui.ui(|ui| Self::signal_plot(ui, &mut state.digital));
                tui.ui(|ui| Self::spectrum_plot(ui, &mut state.harmonic));
                tui.ui(|ui| Self::spectrum_plot(ui, &mut state.digital));
            });
    }

    // ─── Controls panel ─────────────────────────────────────────
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

    // ─── Series helpers ─────────────────────────────────────────
    fn signal_series(plot_ui: &mut PlotUi, signal: &mut SignalState) {
        let (name, color) = match signal.signal_type {
            SignalType::Digital => ("digital_signal", Color32::YELLOW),
            SignalType::Harmonic => ("harmonic_signal", Color32::RED),
        };
        plot_ui.line(Line::new(name, signal.derived.samples.clone()).color(color));
    }

    fn spectrum_series(plot_ui: &mut PlotUi, signal: &mut SignalState) {
        let (name, color) = match signal.signal_type {
            SignalType::Digital => ("digital_spectrum", Color32::LIGHT_BLUE),
            SignalType::Harmonic => ("harmonic_spectrum", Color32::LIGHT_RED),
        };

        let duration = signal.controls.signal_properties.duration;
        let mut df = (1.0 / duration).abs();
        if !df.is_finite() {
            df = 1.0;
        }
        let bar_width = (df * 0.9) as f64;

        let bars: Vec<Bar> = signal
            .derived
            .spectrum
            .iter()
            .map(|[k, amp]| Bar::new(*k, *amp).width(bar_width))
            .collect();

        plot_ui.bar_chart(BarChart::new(name, bars).color(color));
    }

    // ─── Plot constructors ──────────────────────────────────────
    fn signal_plot(ui: &mut Ui, state: &mut SignalState) {
        let name = match state.signal_type {
            SignalType::Digital => "digital",
            SignalType::Harmonic => "harmonic",
        };
        Self::make_plot(name).show(ui, |plot_ui| {
            Self::signal_series(plot_ui, state);
        });
    }

    fn spectrum_plot(ui: &mut Ui, state: &mut SignalState) {
        let name = match state.signal_type {
            SignalType::Digital => "digital spectrum",
            SignalType::Harmonic => "harmonic spectrum",
        };
        Self::make_spectrum_plot(name).show(ui, |plot_ui| {
            Self::spectrum_series(plot_ui, state);
        });
    }

    fn make_plot(name: &str) -> Plot<'_> {
        Plot::new(name)
            .legend(Legend::default().title(name))
            .x_axis_label("time, s")
            .y_axis_label("voltage, V")
    }

    fn make_spectrum_plot(name: &str) -> Plot<'_> {
        Plot::new(name)
            .legend(Legend::default().title(name))
            .x_axis_label("frequency, Hz")
            .y_axis_label("amplitude")
    }
}
