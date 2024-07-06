#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod profile_manager;

// use crate::profile_manager::profile::Profile;
use eframe::egui;
use egui::debug_text::print;

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Ferium Profile Manager",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {
    profile_name: String,
    github_username: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ferium Profile Manager");
            ui.horizontal(|ui| {
                ui.label("Profile Name:");
                ui.label(&self.profile_name);
            });
            ui.horizontal(|ui| {
                ui.label("GitHub Username:");
                ui.add(egui::TextEdit::singleline(&mut self.github_username));
            });
            if ui.button("Save Profile").clicked() {
                self.profile_name = self.github_username.clone();
            }

            if ui.button("Load Profile").clicked() {
                // Implement loading a profile
            }
        });
    }
}
