use chrono::Local;
use arboard::Clipboard;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use tauri::command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Serialize, Deserialize, Clone)]
pub struct ImportResult {
    pub success: bool,
    pub message: String,
    pub processed_mods: Vec<String>,
    pub failed_mods: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExportResult {
    pub success: bool,
    pub message: String,
    pub mod_list: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpgradeResult {
    pub success: bool,
    pub message: String,
    pub output: String,
}

#[command]
pub async fn import_mods(mod_list: Vec<String>) -> Result<ImportResult, String> {
    let mut processed_mods = Vec::new();
    let mut failed_mods = Vec::new();

    for mod_name in mod_list {
        if mod_name.trim().is_empty() {
            continue;
        }

        let mut cmd = Command::new("ferium");
        
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);
        
        cmd.arg("add").arg(&mod_name);
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    
                    // Try to extract the actual mod name from Ferium's output
                    // Expected format: "Successfully added ModName" or similar
                    let actual_mod_name = extract_mod_name_from_output(&stdout)
                        .unwrap_or_else(|| mod_name.clone());
                    
                    processed_mods.push(actual_mod_name);
                } else {
                    failed_mods.push(mod_name.clone());
                }
            }
            Err(_) => {
                failed_mods.push(mod_name.clone());
            }
        }
    }

    let success = failed_mods.is_empty();
    let message = if success {
        format!("✅ Successfully imported {} mods", processed_mods.len())
    } else {
        format!("⚠️ Imported {} mods, {} failed", processed_mods.len(), failed_mods.len())
    };

    Ok(ImportResult {
        success,
        message,
        processed_mods,
        failed_mods,
    })
}

#[command]
pub async fn export_mods() -> Result<ExportResult, String> {
    let mut cmd = Command::new("ferium");
    
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    cmd.arg("list");

    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                let mod_list = extract_middle_text(output_str);
                
                // Copy to clipboard
                match Clipboard::new() {
                    Ok(mut clipboard) => {
                        if let Err(e) = clipboard.set_text(mod_list.clone()) {
                            return Ok(ExportResult {
                                success: false,
                                message: format!("Failed to copy to clipboard: {}", e),
                                mod_list,
                                timestamp: get_current_time(),
                            });
                        }
                    }
                    Err(e) => {
                        return Ok(ExportResult {
                            success: false,
                            message: format!("Failed to access clipboard: {}", e),
                            mod_list,
                            timestamp: get_current_time(),
                        });
                    }
                }

                Ok(ExportResult {
                    success: true,
                    message: "📋 Mod list copied to clipboard".to_string(),
                    mod_list,
                    timestamp: get_current_time(),
                })
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(format!("Ferium command failed: {}", error_message))
            }
        }
        Err(e) => Err(format!("Failed to execute ferium: {}", e)),
    }
}

#[command]
pub async fn check_ferium_available() -> Result<bool, String> {
    let mut cmd = Command::new("ferium");
    
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    cmd.arg("--version");

    match cmd.output() {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Ok(false),
    }
}

#[command]
pub async fn upgrade_mods() -> Result<UpgradeResult, String> {
    let mut cmd = Command::new("ferium");
    
    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);
    
    cmd.arg("upgrade");

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let combined_output = if !stderr.is_empty() && !stdout.is_empty() {
                format!("{}\n{}", stdout, stderr)
            } else if !stderr.is_empty() {
                stderr.clone()
            } else {
                stdout.clone()
            };

            if output.status.success() {
                // Check if the upgrade actually did something or if it was already up to date
                let output_lower = combined_output.to_lowercase();
                let message = if output_lower.contains("all up to date!") {
                    "✅ All mods are already up to date!"
                } else if output_lower.contains("downloading") && output_lower.contains("mod") {
                    "🚀 Successfully downloaded and installed mod updates!"
                } else if output_lower.contains("✓") && (output_lower.contains(".jar") || output_lower.contains("fabric") || output_lower.contains("forge")) {
                    // If we see checkmarks with jar files, mods were processed
                    if output_lower.contains("all up to date") {
                        "✅ All mods checked and confirmed up to date!"
                    } else {
                        "🚀 Successfully processed and updated mods!"
                    }
                } else if output_lower.contains("determining") && output_lower.contains("versions") {
                    "🔍 Successfully checked mod versions!"
                } else if output_lower.contains("no mods") || (output_lower.contains("no") && output_lower.contains("mod")) {
                    "ℹ️ No mods found to upgrade - add some mods first!"
                } else if combined_output.trim().is_empty() {
                    "✅ Upgrade completed successfully!"
                } else {
                    "🚀 Upgrade process completed!"
                };

                Ok(UpgradeResult {
                    success: true,
                    message: message.to_string(),
                    output: combined_output,
                })
            } else {
                Ok(UpgradeResult {
                    success: false,
                    message: "❌ Upgrade failed - check the output below".to_string(),
                    output: combined_output,
                })
            }
        }
        Err(e) => Err(format!("Failed to execute ferium upgrade: {}", e)),
    }
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

fn extract_mod_name_from_output(output: &str) -> Option<String> {
    // Try multiple patterns that Ferium might use
    let patterns = [
        r"Successfully added (.+)",                    // "Successfully added Zoomify"
        r"Added (.+) successfully",                    // Alternative format
        r"✓ Added (.+)",                              // With checkmark
        r"Added: (.+)",                               // With colon
        r"The project has already been added: (.+)",   // Already added case
        r"Already added: (.+)",                       // Alternative already added
        r"(.+) has already been added",               // Another variant
        r"(.+) is already installed",                 // Another variant
    ];
    
    for pattern in &patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            if let Some(captures) = regex.captures(output) {
                if let Some(mod_name) = captures.get(1) {
                    return Some(mod_name.as_str().trim().to_string());
                }
            }
        }
    }
    
    // If no pattern matches, try to find any line that looks like it contains the mod name
    // Look for lines that aren't just status messages
    for line in output.lines() {
        let line = line.trim();
        if !line.is_empty() 
            && !line.starts_with("Downloading") 
            && !line.starts_with("Fetching")
            && !line.contains("bytes")
            && line.len() > 3 
        {
            // This might be our mod name line, extract the meaningful part
            if let Some(meaningful_part) = extract_meaningful_part(line) {
                return Some(meaningful_part);
            }
        }
    }
    
    None
}

fn extract_meaningful_part(line: &str) -> Option<String> {
    // Remove common prefixes and extract the mod name
    let prefixes_to_remove = ["Successfully added", "Added", "✓", "-", "•"];
    
    let mut cleaned = line.to_string();
    for prefix in &prefixes_to_remove {
        if cleaned.starts_with(prefix) {
            cleaned = cleaned[prefix.len()..].trim().to_string();
        }
    }
    
    // Remove version numbers and extra info (anything in parentheses or after certain chars)
    if let Some(pos) = cleaned.find('(') {
        cleaned = cleaned[..pos].trim().to_string();
    }
    if let Some(pos) = cleaned.find('[') {
        cleaned = cleaned[..pos].trim().to_string();
    }
    
    if !cleaned.is_empty() && cleaned.len() > 1 {
        Some(cleaned)
    } else {
        None
    }
}