use std::process::Command;
use std::fs::File;
use std::io::Write;
use git2::Repository;
use std::path::Path;

use super::profile::Profile;

impl Profile {
    pub fn save_profile(&self, output_dir: String) -> Result<(), Box<dyn std::error::Error>> {
        // Run ferium list and save to profile.txt
        let output = Command::new("ferium")
            .arg("list")
            .output()?;
        
        
        let profile_dir = output_dir;
        let profile_path = format!("{}/profile.txt", profile_dir);
        
        let mut file = File::create(&profile_path)?;
        file.write_all(&output.stdout)?;
        
        // // Commit and push to Git repository
        // let repo = Repository::open(profile_dir)?;
        // let mut index = repo.index()?;
        // index.add_path(Path::new("profile.txt"))?;
        // index.write()?;
        
        // let oid = index.write_tree()?;
        // let signature = repo.signature()?;
        // let parent_commit = repo.head()?.peel_to_commit()?;
        // let tree = repo.find_tree(oid)?;
        
        // repo.commit(
        //     Some("HEAD"),
        //     &signature,
        //     &signature,
        //     "Save profile",
        //     &tree,
        //     &[&parent_commit]
        // )?;
        
        // let mut remote = repo.find_remote("origin")?;
        // remote.push(&["refs/heads/main:refs/heads/main"], None)?;
        
        Ok(())
    }
}
