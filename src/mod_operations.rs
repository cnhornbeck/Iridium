use chrono::Local;

use arboard::Clipboard;

use std::{
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

// use crate::github_integration::repo_content::RepoContent;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

// pub async fn import_from_repo(
//     debug_string: Arc<Mutex<String>>,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let content = RepoContent::fetch_repo_contents().await?;

//     let input_string: Vec<String> = content.split('\n').map(|s| s.to_string()).collect();

//     import_mods(input_string, debug_string);

//     Ok(content)
// }

pub fn import_mods(import_string: Vec<String>, debug_string: Arc<Mutex<String>>) {
    let mut errors = Vec::new();

    for line in import_string {
        if line.is_empty() {
            continue;
        }
        let mut cmd = Command::new("Ferium");

        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.arg("add").arg(&line);

        // Pipe stdout and stderr
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    let success_message = String::from_utf8_lossy(&output.stdout).into_owned();
                    let mut debug_string_lock = debug_string.lock().unwrap();
                    *debug_string_lock = success_message;
                } else {
                    let error_message = String::from_utf8_lossy(&output.stderr);
                    let error_msg = format!("Command failed for mod '{}': {}", line, error_message);
                    errors.push(error_msg.clone());
                    let mut debug_string_lock = debug_string.lock().unwrap();
                    *debug_string_lock = error_msg;
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to execute command for mod '{}': {}", line, e);
                errors.push(error_msg.clone());
                let mut debug_string_lock = debug_string.lock().unwrap();
                *debug_string_lock = error_msg;
            }
        }
    }
}

pub fn export_mods() -> Result<String, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("Ferium");

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    cmd.arg("list");

    let output = cmd.output()?;
    let output_str = extract_middle_text(String::from_utf8_lossy(&output.stdout).to_string());

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(output_str.clone()).unwrap();

    let current_time = get_current_time();
    let export_status = if output.status.success() {
        format!("Copied mods at {}", current_time)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        format!("Command failed: {}", error_message)
    };

    Ok(export_status)
}

fn extract_middle_text(input: String) -> String {
    input
        .lines()
        .skip(1) // Skip the first line as it tells us how many mods are installed
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_current_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}
