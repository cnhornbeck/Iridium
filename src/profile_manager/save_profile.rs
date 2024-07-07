use std::process::Command;

use super::profile::Profile;

impl Profile {
    pub async fn save_profile(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Run ferium list and save to profile.txt
        let output = Command::new("ferium").arg("list").output()?;

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(output_str)
    }
}
