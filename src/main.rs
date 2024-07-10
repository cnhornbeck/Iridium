// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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

#![windows_subsystem = "windows"]

mod github_integration;
mod profile_manager;
use eframe::{egui, App, Frame};
use egui::{Context, FontId, Pos2, RichText, Rounding, Shadow, Stroke, TextEdit, Window};
use profile_manager::profile::Profile;
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
    debug_string: String,
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

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let mut state = self.state.lock().unwrap();
        let fixed_size = egui::vec2(200.0, 200.0);

        self.render_left_panel(ctx, &mut state, fixed_size);
        self.render_central_panel(ctx);
    }
}

impl MyApp {
    fn render_left_panel(&self, ctx: &Context, state: &mut AppState, fixed_size: egui::Vec2) {
        let panel_width = ctx.input(|i| i.screen_rect().width()) / 2.0;

        egui::SidePanel::left("left_panel")
            .exact_width(panel_width)
            .resizable(false)
            .show(ctx, |ui| {
                self.render_panel_header(ui);
                self.render_import_export_buttons(ctx, ui, state);
                self.render_windows(ctx, ui, fixed_size, panel_width, state);
            });
    }

    fn render_panel_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Command Center").font(FontId::proportional(25.0)));
        });
    }

    fn render_import_export_buttons(&self, ctx: &Context, ui: &mut egui::Ui, state: &mut AppState) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            ui.add_space(5.0);

            if ui.button("Import").clicked() {
                match Profile::import_mods(&state.import_string, &mut state.debug_string) {
                    Ok(_) => println!(
                        "All mods imported successfully. Last operation: {}",
                        state.debug_string
                    ),
                    Err(e) => {
                        eprintln!("Errors occurred during import: {}", e);
                        println!("Last operation: {}", state.debug_string);
                    }
                }
            }

            let spacing = ctx.input(|i| i.screen_rect().width()) / 4.0 - 60.0;
            ui.add_space(spacing);

            if ui.button("Export").clicked() {
                state.export_string = "This is a test export string".to_string();
            }
        });
    }

    fn render_windows(
        &self,
        ctx: &Context,
        ui: &mut egui::Ui,
        fixed_size: egui::Vec2,
        panel_width: f32,
        state: &mut AppState,
    ) {
        ui.horizontal(|ui| {
            self.set_window_style(ctx, ui);
            self.render_export_window(ctx, ui, fixed_size, state, panel_width);
            self.render_debug_window(ctx, ui, fixed_size, state);
            self.render_import_window(ctx, ui, fixed_size, state);
        });
    }

    fn set_window_style(&self, ctx: &Context, ui: &mut egui::Ui) {
        ui.style_mut().visuals.window_shadow = Shadow::NONE;
        ui.style_mut().visuals.window_rounding = Rounding::ZERO;
        ui.style_mut().visuals.window_stroke = Stroke::NONE;
        ctx.set_style(ui.style().clone());
    }

    fn render_export_window(
        &self,
        ctx: &Context,
        ui: &mut egui::Ui,
        fixed_size: egui::Vec2,
        state: &mut AppState,
        panel_width: f32,
    ) {
        let x_pos = panel_width / 2.0 + (panel_width / 2.0 - fixed_size.x) / 2.0;

        Window::new("Export")
            .movable(false)
            .open(&mut true)
            .current_pos(Pos2::new(x_pos, 60.0))
            .fixed_size(fixed_size)
            .resizable(false)
            .title_bar(false)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label(state.export_string.clone());
            });
    }

    fn render_import_window(
        &self,
        ctx: &Context,
        ui: &mut egui::Ui,
        fixed_size: egui::Vec2,
        state: &mut AppState,
    ) {
        Window::new("Import")
            .movable(false)
            .open(&mut true)
            .current_pos(Pos2::new(6.0, 60.0))
            .fixed_size(fixed_size)
            .resizable(false)
            .title_bar(false)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.add(TextEdit::multiline(&mut state.import_string));
            });
    }

    fn render_debug_window(
        &self,
        ctx: &Context,
        ui: &mut egui::Ui,
        fixed_size: egui::Vec2,
        state: &mut AppState,
    ) {
        Window::new("Debug")
            .movable(false)
            .open(&mut true)
            .current_pos(Pos2::new(6.0, 265.0))
            .fixed_size(fixed_size)
            .resizable(false)
            .title_bar(false)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.label(state.debug_string.clone());
            });
    }

    fn render_central_panel(&self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Coming Soon!").font(FontId::proportional(40.0)));
            });
        });
    }
}
