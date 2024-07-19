use super::profile::Profile;

use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

impl Profile {
    pub fn import_mods(import_string: &str, debug_string: &mut String) -> Result<(), String> {
        let mut errors = Vec::new();
        let mut temp_debug = String::new();

        let mut cmd = Command::new("Ferium");
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        for line in import_string.lines() {
            // println!("Importing mod: {}", line);

            cmd.arg("add").arg(line);

            // Hide console window on Windows

            // Pipe stdout and stderr
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            let output = match cmd.output() {
                Ok(output) => output,
                Err(e) => {
                    let error_msg = format!("Failed to execute command for mod '{}': {}", line, e);
                    errors.push(error_msg.clone());
                    temp_debug = error_msg;
                    continue;
                }
            };

            if !output.status.success() {
                let error_message = String::from_utf8_lossy(&output.stderr);
                let error_msg = format!("Command failed for mod '{}': {}", line, error_message);
                errors.push(error_msg.clone());
                temp_debug = error_msg;
            } else {
                let success_message = String::from_utf8_lossy(&output.stdout);
                // println!("Success: {}", success_message);
                temp_debug = format!("Successfully imported mod: {}", line);
            }
        }
        cmd.arg("upgrade");

        // Overwrite debug_string with the last message
        *debug_string = temp_debug;

        if !errors.is_empty() {
            Err(format!(
                "Errors occurred during import:\n{}",
                errors.join("\n")
            ))
        } else {
            Ok(())
        }
    }
}
