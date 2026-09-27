use eframe::egui::{self, ViewportBuilder};

use crate::ui::{State};

mod generator;
mod ui;
mod guarded;

#[derive(Default)]
struct App {
    state: State,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            self.ui(ui)
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_title("Lab-1")
            .with_inner_size([960.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lab-1",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(App::default()))
        }),
    )
}