use super::profile::Profile;
use crate::github_integration::repo_content::RepoContent;

impl Profile {
    pub async fn load_profile() -> Result<String, Box<dyn std::error::Error>> {
        let content = RepoContent::fetch_repo_contents().await?;

        // let output = Command::new("ferium").arg("upgrade").output()?;

        // let output_str = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(content)
    }
}
