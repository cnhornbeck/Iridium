// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// mod github_integration;
// mod profile_manager;
// use eframe::egui;
// use profile_manager::profile::Profile;
// use std::sync::Arc;
// use tokio::runtime::Runtime;

// fn main() -> eframe::Result {
//     let options = eframe::NativeOptions {
//         viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
//         ..Default::default()
//     };
//     eframe::run_native(
//         "Ferium Profile Manager",
//         options,
//         Box::new(|cc| {
//             // This gives us image support:
//             egui_extras::install_image_loaders(&cc.egui_ctx);

//             Ok(Box::<MyApp>::default())
//         }),
//     )
// }

// #[derive(Default)]
// struct MyApp {
//     profile_name: String,
//     github_username: String,
//     load_profile_output: String,
//     profile: Option<Profile>,
// }

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         let rt = Arc::new(Runtime::new().unwrap());

//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Ferium Profile Manager");
//             ui.horizontal(|ui| {
//                 ui.label("Profile Name:");
//                 ui.text_edit_singleline(&mut self.profile_name);
//             });
//             ui.horizontal(|ui| {
//                 ui.label("GitHub Username:");
//                 ui.text_edit_singleline(&mut self.github_username);
//             });
//             if ui.button("Save Profile").clicked() {
//                 let profile_name = self.profile_name.clone();
//                 let rt_clone = rt.clone();
//                 let profile = self.profile.clone();

//                 rt.spawn(async move {
//                     if let Some(prof) = profile {
//                         match prof.save_profile().await {
//                             Ok(output) => {
//                                 // Update the profile_name with the saved output
//                                 // You'll need to use a mutex or channel to update the UI safely
//                             }
//                             Err(e) => {
//                                 eprintln!("Error saving profile: {}", e);
//                             }
//                         }
//                     } else {
//                         let new_profile = Profile::new(profile_name, vec![]);
//                         match new_profile.save_profile().await {
//                             Ok(output) => {
//                                 // Update the UI safely
//                             }
//                             Err(e) => {
//                                 eprintln!("Error saving new profile: {}", e);
//                             }
//                         }
//                     }
//                 });
//             }

//             ui.vertical(|ui| {
//                 ui.label("This will load the profile from the local system");
//                 ui.horizontal(|ui| {
//                     if ui.button("Load Profile").clicked() {
//                         let rt_clone = rt.clone();
//                         rt.spawn(async move {
//                             match Profile::load_profile().await {
//                                 Ok(output) => {
//                                     // Update the UI safely
//                                 }
//                                 Err(e) => {
//                                     eprintln!("Error loading profile: {}", e);
//                                 }
//                             }
//                         });
//                     };
//                     ui.label(&self.load_profile_output);
//                     ui.separator();
//                 });
//             });

//             // Display current profile information if available
//             if let Some(profile) = &self.profile {
//                 ui.label(format!("Current Profile: {}", profile.name));
//                 ui.label(format!("Number of mods: {}", profile.mods.len()));
//             }
//         });
//     }
// }

use eframe::egui;
use egui::{vec2, FontId, RichText, Rounding, Shadow, Stroke, TextEdit, Ui, Window};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([960.0, 540.0])
            .with_position([0.0, 0.0]),

        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            // let state = Arc::new(Mutex::new(AppState::default()));
            Ok(Box::new(MyApp::new()))
        }),
    )
}
#[derive(Default)]
struct AppState {
    import_string: String,
    export_string: String,
}

struct MyApp {
    state: Arc<Mutex<AppState>>,
}

impl MyApp {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(AppState::default())),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut state = self.state.lock().unwrap();
        let fixed_size = vec2(200.0, 350.0);

        egui::SidePanel::left("left_panel")
            .exact_width(ctx.input(|i: &egui::InputState| i.screen_rect()).width() / 2.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Command Center").font(FontId::proportional(25.0)));
                });

                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    Ui::add_space(ui, 5.0);

                    if ui.button("Import").clicked() {
                        state.import_string = "This is a test import string".to_string();
                    }

                    Ui::add_space(
                        ui,
                        ctx.input(|i: &egui::InputState| i.screen_rect()).width() / 4.0
                            + (ctx.input(|i: &egui::InputState| i.screen_rect()).width() / 4.0
                                - fixed_size.x)
                                / 2.0
                            - 60.0,
                    );

                    if ui.button("Export").clicked() {
                        state.export_string = "This is a test export string".to_string();
                    }
                });

                ui.horizontal(|ui| {
                    ui.style_mut().visuals.window_shadow = Shadow::NONE;
                    ui.style_mut().visuals.window_rounding = Rounding::ZERO;
                    ui.style_mut().visuals.window_stroke = Stroke::NONE;
                    ctx.set_style(ui.style().clone());
                    Window::new("Import")
                        .movable(false)
                        .open(&mut true)
                        .current_pos(egui::Pos2::new(ui.next_widget_position().x, 65.0))
                        .fixed_size(fixed_size)
                        .resizable(false)
                        .title_bar(false)
                        .vscroll(true)
                        .show(ctx, |ui| {
                            ui.label("This is a test label");
                        });

                    Window::new("Export")
                        .movable(false)
                        .open(&mut true)
                        .current_pos(egui::Pos2::new(
                            ctx.input(|i: &egui::InputState| i.screen_rect()).width() / 4.0
                                + (ctx.input(|i: &egui::InputState| i.screen_rect()).width() / 4.0
                                    - fixed_size.x)
                                    / 2.0,
                            65.0,
                        ))
                        .fixed_size(fixed_size)
                        .resizable(false)
                        .title_bar(false)
                        .vscroll(true)
                        .show(ctx, |ui| {
                            ui.add(TextEdit::multiline(&mut "my lines\n".repeat(100)));
                        });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Coming Soon!").font(FontId::proportional(40.0)));
            });
        });
    }
}
