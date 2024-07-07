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
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct AppState {
    query_result: Option<Vec<String>>,
    is_loading: bool,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            let state = Arc::new(Mutex::new(AppState::default()));
            Ok(Box::new(MyEguiApp::new(state)))
        }),
    )
}

struct MyEguiApp {
    state: Arc<Mutex<AppState>>,
}

impl MyEguiApp {
    fn new(state: Arc<Mutex<AppState>>) -> Self {
        Self { state }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut state = self.state.lock().unwrap();

            if !state.is_loading
                && state.query_result.is_none()
                && ui.button("Start Query").clicked()
            {
                state.is_loading = true;
                let state_clone = self.state.clone();
                thread::spawn(move || {
                    // Simulate a long-running query
                    thread::sleep(std::time::Duration::from_secs(1));
                    let result = vec!["mod1".to_string(), "mod2".to_string()];
                    let mut state = state_clone.lock().unwrap();
                    state.query_result = Some(result);
                    state.is_loading = false;
                });
            }

            if state.is_loading {
                ui.label("Loading...");
            } else if let Some(result) = &state.query_result {
                ui.label("Query Result:");
                for mod_id in result {
                    ui.label(mod_id);
                }
            }
        });

        // Request a repaint on each frame to handle updates from the background thread
        ctx.request_repaint();
    }
}
