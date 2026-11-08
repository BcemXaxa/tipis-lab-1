use std::f64::consts::TAU;

use eframe::egui::{self, Color32, Ui, Window};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotUi};
use egui_taffy::{TuiBuilderLogic, taffy, tui};
use modern_egui::theme::{TextColor, TextSize, UiText};
use taffy::{Style, prelude::*};

use crate::{
    App,
    derivation::Derived,
    generator::{SignalProperties, SignalType, WaveProperties},
    modulation::{ModulationSettings, ModulationType},
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum PropertyPreset {
    Default,
    Carrying,
    Modulating1,
    Modulating2,
}

impl PropertyPreset {
    const ALL: [Self; 4] = [Self::Default, Self::Carrying, Self::Modulating1, Self::Modulating2];

    fn label(self) -> &'static str {
        use PropertyPreset as P;
        match self {
            P::Default => "Default",
            P::Carrying => "Carrying",
            P::Modulating1 => "Modulating 1",
            P::Modulating2 => "Modulating 2",
        }
    }

    fn matches(self, wave: &WaveProperties) -> bool {
        self.wave_properties() == *wave
    }

    fn apply(self, wave: &mut WaveProperties) {
        *wave = self.wave_properties();
    }

    fn wave_properties(self) -> WaveProperties {
        use PropertyPreset as P;
        match self {
            P::Default => WaveProperties {
                frequency: 4.0,
                amplitude: 1.0,
                phase: 0.0,
            },
            P::Carrying => WaveProperties {
                frequency: 24.0,
                amplitude: 1.0,
                phase: 0.0,
            },
            P::Modulating1 => WaveProperties {
                frequency: 2.0,
                amplitude: 1.0,
                phase: 0.0,
            },
            P::Modulating2 => WaveProperties {
                frequency: 4.0,
                amplitude: 1.0,
                phase: 0.25,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PlotKind {
    Harmonic,
    Digital,
    Am,
    Fm,
    Pm,
}

impl PlotKind {
    fn signal_style(self) -> (&'static str, Color32) {
        match self {
            Self::Harmonic => ("harmonic_signal", Color32::from_rgb(255, 40, 40)),
            Self::Digital => ("digital_signal", Color32::from_rgb(80, 110, 255)),
            Self::Am => ("am_signal", Color32::from_rgb(90, 220, 90)),
            Self::Fm => ("fm_signal", Color32::from_rgb(90, 200, 230)),
            Self::Pm => ("pm_signal", Color32::from_rgb(220, 120, 220)),
        }
    }

    fn spectrum_style(self) -> (&'static str, Color32) {
        match self {
            Self::Harmonic => ("harmonic_spectrum", Color32::from_rgb(255, 120, 120)),
            Self::Digital => ("digital_spectrum", Color32::from_rgb(160, 180, 255)),
            Self::Am => ("am_spectrum", Color32::from_rgb(150, 255, 150)),
            Self::Fm => ("fm_spectrum", Color32::from_rgb(150, 230, 255)),
            Self::Pm => ("pm_spectrum", Color32::from_rgb(255, 160, 255)),
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Harmonic => "harmonic",
            Self::Digital => "digital",
            Self::Am => "AM",
            Self::Fm => "FM",
            Self::Pm => "PM",
        }
    }

    fn spectrum_title(self) -> &'static str {
        match self {
            Self::Harmonic => "harmonic spectrum",
            Self::Digital => "digital spectrum",
            Self::Am => "AM spectrum",
            Self::Fm => "FM spectrum",
            Self::Pm => "PM spectrum",
        }
    }
}

struct SignalState {
    kind: PlotKind,
    controls: Controls,
    derived: Derived,
}

impl SignalState {
    fn new(kind: PlotKind) -> Self {
        let controls = Controls::default();
        let derived = match kind {
            PlotKind::Harmonic => Derived::new(
                SignalType::Harmonic,
                controls.wave_properties.clone(),
                controls.signal_properties.clone(),
            ),
            PlotKind::Digital => Derived::new(
                SignalType::Digital,
                controls.wave_properties.clone(),
                controls.signal_properties.clone(),
            ),
            PlotKind::Am | PlotKind::Fm | PlotKind::Pm => Derived::empty(),
        };
        Self {
            kind,
            controls,
            derived,
        }
    }

    fn get_controls(&self) -> Controls {
        self.controls.clone()
    }

    fn set_controls(&mut self, controls: Controls) -> bool {
        if self.controls != controls {
            match self.kind {
                PlotKind::Harmonic => {
                    self.derived = Derived::new(
                        SignalType::Harmonic,
                        controls.wave_properties.clone(),
                        controls.signal_properties.clone(),
                    );
                }
                PlotKind::Digital => {
                    self.derived = Derived::new(
                        SignalType::Digital,
                        controls.wave_properties.clone(),
                        controls.signal_properties.clone(),
                    );
                }
                PlotKind::Am | PlotKind::Fm | PlotKind::Pm => {}
            }
            self.controls = controls;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Tab {
    #[default]
    Signals,
    Modulation,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ConfigTab {
    #[default]
    Harmonic,
    Digital,
    Modulation,
}

pub struct State {
    harmonic: SignalState,
    digital: SignalState,
    modulated: [SignalState; 3],
    modulation: ModulationSettings,
    tab: Tab,
    config_tab: ConfigTab,
    controls_open: bool,
}

impl Default for State {
    fn default() -> Self {
        let mut state = Self {
            harmonic: SignalState::new(PlotKind::Harmonic),
            digital: SignalState::new(PlotKind::Digital),
            modulated: [
                SignalState::new(PlotKind::Am),
                SignalState::new(PlotKind::Fm),
                SignalState::new(PlotKind::Pm),
            ],
            modulation: ModulationSettings::default(),
            tab: Tab::default(),
            config_tab: ConfigTab::default(),
            controls_open: false,
        };
        state.refresh_modulated();
        state
    }
}

impl State {
    fn refresh_modulated(&mut self) {
        let carrier = self.harmonic.controls.wave_properties.clone();
        let modulating = self.digital.controls.wave_properties.clone();
        let signal_properties = self.harmonic.controls.signal_properties.clone();

        for (signal, kind) in self.modulated.iter_mut().zip(ModulationType::ALL) {
            signal.controls.signal_properties = signal_properties.clone();
            signal.derived = Derived::new_modulated(
                carrier.clone(),
                modulating.clone(),
                kind,
                self.modulation.index(kind),
                signal_properties.clone(),
            );
        }
    }
}

// ─── Layout ─────────────────────────────────────────────────────
impl App {
    pub fn ui(ui: &mut Ui, state: &mut State) {
        // ── Toolbar ─────────────────────────────────────────────
        ui.horizontal(|ui| {
            ui.selectable_value(&mut state.tab, Tab::Signals, "Signals");
            ui.selectable_value(&mut state.tab, Tab::Modulation, "Modulation");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(format!("{} Configure", egui_phosphor::regular::GEAR))
                    .clicked()
                {
                    state.controls_open = !state.controls_open;
                }
            });
        });
        ui.separator();

        // ── Floating settings window ────────────────────────────
        if state.controls_open {
            let mut changed = false;
            Window::new("Configuration")
                .open(&mut state.controls_open)
                .resizable(true)
                //.default_size([560.0, 320.0])
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut state.config_tab, ConfigTab::Harmonic, "Harmonic");
                        ui.selectable_value(&mut state.config_tab, ConfigTab::Digital, "Digital");
                        ui.selectable_value(
                            &mut state.config_tab,
                            ConfigTab::Modulation,
                            "Modulation",
                        );
                    });
                    ui.separator();

                    match state.config_tab {
                        ConfigTab::Harmonic => {
                            let mut c = state.harmonic.get_controls();
                            Self::controls_panel(ui, "harmonic", &mut c);
                            changed |= state.harmonic.set_controls(c);
                        }
                        ConfigTab::Digital => {
                            let mut c = state.digital.get_controls();
                            Self::controls_panel(ui, "digital", &mut c);
                            changed |= state.digital.set_controls(c);
                        }
                        ConfigTab::Modulation => {
                            let mut m = state.modulation;
                            Self::modulation_panel(ui, &mut m);
                            if m != state.modulation {
                                state.modulation = m;
                                changed = true;
                            }
                        }
                    }
                });
            if changed {
                state.refresh_modulated();
            }
        }

        // ── Plot grid ───────────────────────────────────────────
        match state.tab {
            Tab::Signals => {
                tui(ui, ui.id().with("signals_grid"))
                    .reserve_available_space()
                    .style(Self::grid_style(2))
                    .show(|tui| {
                        tui.ui(|ui| Self::signal_plot(ui, &mut state.harmonic));
                        tui.ui(|ui| Self::signal_plot(ui, &mut state.digital));
                        tui.ui(|ui| Self::spectrum_plot(ui, &mut state.harmonic));
                        tui.ui(|ui| Self::spectrum_plot(ui, &mut state.digital));
                    });
            }
            Tab::Modulation => {
                tui(ui, ui.id().with("modulation_grid"))
                    .reserve_available_space()
                    .style(Self::grid_style(3))
                    .show(|tui| {
                        for signal in &mut state.modulated {
                            tui.ui(|ui| Self::signal_plot(ui, signal));
                        }
                        for signal in &mut state.modulated {
                            tui.ui(|ui| Self::spectrum_plot(ui, signal));
                        }
                    });
            }
        }
    }

    fn grid_style(columns: usize) -> Style {
        use taffy::style_helpers::{fr, length, percent};
        Style {
            display: taffy::Display::Grid,
            grid_template_columns: (0..columns).map(|_| fr(1.0f32)).collect(),
            grid_template_rows: vec![fr(1.0f32), fr(1.0f32)],
            gap: Size {
                width: length(8.0f32),
                height: length(8.0f32),
            },
            size: percent(1.0f32),
            align_items: Some(AlignItems::Stretch),
            justify_items: Some(AlignItems::Stretch),
            ..Default::default()
        }
    }

    // ─── Controls panel ─────────────────────────────────────────
    fn reset_button(ui: &mut Ui, tooltip: &str) -> bool {
        let size = egui::vec2(ui.spacing().interact_size.y, ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        let hovered = response.hovered();

        let (icon, family) = if hovered {
            (
                egui_phosphor::bold::ARROWS_CLOCKWISE,
                egui::FontFamily::Name("phosphor-bold".into()),
            )
        } else {
            (
                egui_phosphor::regular::ARROWS_CLOCKWISE,
                egui::FontFamily::Proportional,
            )
        };

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::new(16.0, family),
            ui.visuals().widgets.inactive.fg_stroke.color,
        );

        response.on_hover_text(tooltip).clicked()
    }

    fn controls_panel(ui: &mut Ui, id: &str, controls: &mut Controls) {
        let Controls {
            wave_properties: w,
            signal_properties: s,
        } = controls;

        let selected = PropertyPreset::ALL
            .into_iter()
            .find(|preset| preset.matches(w));

        ui.columns(2, |cols| {
            // ── Wave properties ─────────────────────────────────
            cols[0].horizontal(|ui| {
                ui.text_colored("Wave properties", TextSize::CardTitle, TextColor::Muted);
                if PropertyPreset::Default.matches(w) {
                    let _ = ui.allocate_space(egui::vec2(0.0, ui.spacing().interact_size.y));
                } else if Self::reset_button(ui, "Reset wave properties") {
                    PropertyPreset::Default.apply(w);
                }
            });

            egui::Grid::new(("wave_properties", id))
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(&mut cols[0], |ui| {
                    ui.label("preset:");
                    egui::ComboBox::from_id_salt(("preset", id))
                        .width(120.0)
                        .selected_text(selected.map_or("Custom", |preset| preset.label()))
                        .show_ui(ui, |ui| {
                            for preset in PropertyPreset::ALL {
                                if ui
                                    .selectable_label(Some(preset) == selected, preset.label())
                                    .clicked()
                                {
                                    preset.apply(w);
                                }
                            }
                        });
                    ui.end_row();

                    ui.label("frequency:");
                    ui.add(egui::DragValue::new(&mut w.frequency).speed(0.1));
                    ui.end_row();

                    ui.label("amplitude:");
                    ui.add(egui::DragValue::new(&mut w.amplitude).speed(0.1));
                    ui.end_row();

                    ui.label("phase:");
                    let mut phase_tmp = (w.phase * TAU) as f32;
                    if ui.drag_angle_tau(&mut phase_tmp).changed() {
                        w.phase = (phase_tmp as f64 / TAU).rem_euclid(1.0);
                    }
                    ui.end_row();
                });

            // ── Signal properties ───────────────────────────────
            cols[1].horizontal(|ui| {
                ui.text_colored("Signal properties", TextSize::CardTitle, TextColor::Muted);
                let _ = ui.allocate_space(egui::vec2(0.0, ui.spacing().interact_size.y));
            });

            egui::Grid::new(("signal_properties", id))
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(&mut cols[1], |ui| {
                    ui.label("duration:");
                    ui.add(egui::DragValue::new(&mut s.duration).speed(0.5));
                    ui.end_row();

                    ui.label("sample rate:");
                    ui.add(egui::DragValue::new(&mut s.sample_rate).speed(2.0));
                    ui.end_row();
                });
        });
    }

    fn modulation_panel(ui: &mut Ui, settings: &mut ModulationSettings) {
        ui.horizontal(|ui| {
            ui.text_colored("Modulation indices", TextSize::CardTitle, TextColor::Muted);
            if *settings == ModulationSettings::default() {
                let _ = ui.allocate_space(egui::vec2(0.0, ui.spacing().interact_size.y));
            } else if Self::reset_button(ui, "Reset modulation indices") {
                *settings = ModulationSettings::default();
            }
        });

        egui::Grid::new("modulation_indices")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label("AM index:");
                ui.add(egui::DragValue::new(&mut settings.am_index).speed(0.05));
                ui.end_row();

                ui.label("FM index:");
                ui.add(egui::DragValue::new(&mut settings.fm_index).speed(0.1));
                ui.end_row();

                ui.label("PM index:");
                ui.add(egui::DragValue::new(&mut settings.pm_index).speed(0.01));
                ui.end_row();
            });
    }

    // ─── Series helpers ─────────────────────────────────────────
    fn signal_series(plot_ui: &mut PlotUi, signal: &mut SignalState) {
        let (name, color) = signal.kind.signal_style();
        plot_ui.line(Line::new(name, signal.derived.samples.clone()).color(color));
    }

    fn spectrum_series(plot_ui: &mut PlotUi, signal: &mut SignalState) {
        let (name, color) = signal.kind.spectrum_style();

        let duration = signal.controls.signal_properties.duration;
        let mut df = (1.0 / duration).abs();
        if !df.is_finite() {
            df = 1.0;
        }
        let bar_width = (df * 0.93) as f64;

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
        let name = state.kind.title();
        Self::make_plot(name).show(ui, |plot_ui| {
            Self::signal_series(plot_ui, state);
        });
    }

    fn spectrum_plot(ui: &mut Ui, state: &mut SignalState) {
        let name = state.kind.spectrum_title();
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
