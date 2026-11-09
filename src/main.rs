use eframe::egui::{self, ViewportBuilder};
use modern_egui::theme;

use crate::ui::State;

mod derivation;
mod generator;
mod modulation;
mod synth;
mod ui;

#[derive(Default)]
struct App {
    state: State,
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| Self::ui(ui, &mut self.state));
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder::default()
            .with_fullscreen(true)
            .with_title("Lab-1 / Lab-2")
            .with_inner_size([1800.0, 1000.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lab-1 / Lab-2",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            egui_phosphor::add_font_bytes_as_family(
                &mut fonts,
                "phosphor-bold",
                egui_phosphor::Variant::Bold.font_bytes(),
            );
            cc.egui_ctx.set_fonts(fonts);

            theme::apply(&cc.egui_ctx);
            cc.egui_ctx.set_theme(egui::ThemePreference::Dark);
            Ok(Box::new(App::default()))
        }),
    )
}
