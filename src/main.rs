use eframe::egui::{self, ViewportBuilder};
use modern_egui::theme;

use crate::ui::State;

mod derivation;
mod generator;
mod modulation;
mod ui;

#[derive(Default)]
struct App {
    state: State,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| Self::ui(ui, &mut self.state));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_title("Lab-1 / Lab-2")
            .with_inner_size([1800.0, 1000.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lab-1 / Lab-2",
        options,
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
            Ok(Box::new(App::default()))
        }),
    )
}
