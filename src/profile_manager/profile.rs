// src/profile/profile.rs

use std::{
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use crate::github_integration::repo_content::RepoContent;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub mods: Vec<String>,
}

impl Profile {
    pub fn new(name: String, mods: Vec<String>) -> Profile {
        Profile { name, mods }
    }

    pub async fn save_profile(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Run ferium list and save to profile.txt
        let output = Command::new("ferium").arg("list").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(output_str)
    }

    pub async fn load_profile() -> Result<String, Box<dyn std::error::Error>> {
        let content = RepoContent::fetch_repo_contents().await?;

        // let output = Command::new("ferium").arg("upgrade").output()?;

        // let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(content)
    }

    pub fn import_mods(import_string: Vec<String>, debug_string: Arc<Mutex<String>>) {
        let mut errors = Vec::new();

        for line in import_string {
            // {
            //     // Lock the debug_string and update it
            //     let mut debug_string_lock = debug_string.lock().unwrap();
            //     *debug_string_lock = format!("Importing mod: {}", line);
            // }

            let mut cmd = Command::new("Ferium");
            #[cfg(target_os = "windows")]
            cmd.creation_flags(CREATE_NO_WINDOW);

            cmd.arg("add").arg(line.clone());

            // Pipe stdout and stderr
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            let output = match cmd.output() {
                Ok(output) => output,
                Err(e) => {
                    let error_msg = format!("Failed to execute command for mod '{}': {}", line, e);
                    {
                        // Lock the debug_string and update it
                        let mut debug_string_lock = debug_string.lock().unwrap();
                        *debug_string_lock = error_msg.clone();
                    }
                    errors.push(error_msg);
                    continue;
                }
            };

            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr);
                let error_msg = format!("Command failed for mod '{}': {}", line, error_message);
                {
                    // Lock the debug_string and update it
                    let mut debug_string_lock = debug_string.lock().unwrap();
                    *debug_string_lock = error_msg.clone();
                }
                errors.push(error_msg);
            } else {
                let success_message = String::from_utf8(output.stdout).unwrap().to_string();
                {
                    // Lock the debug_string and update it
                    let mut debug_string_lock = debug_string.lock().unwrap();
                    *debug_string_lock = success_message;
                }
            }
        }
    }
}
