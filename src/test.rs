use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Ferium Profile Manager",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    profile_name: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            profile_name: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ferium Profile Manager");

            ui.horizontal(|ui| {
                ui.label("Profile Name:");
                ui.text_edit_singleline(&mut self.profile_name);
            });

            if ui.button("Save Profile").clicked() {
                // Code to save profile
            }

            if ui.button("Load Profile").clicked() {
                // Code to load profile
            }
        });
    }
}
