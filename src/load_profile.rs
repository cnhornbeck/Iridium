use git2::Repository;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile_dir = "/path/to/profiles";

    // Clone or pull latest from repo
    if !Path::new(profile_dir).exists() {
        Repository::clone("https://github.com/username/repo.git", profile_dir)?;
    } else {
        let repo = Repository::open(profile_dir)?;
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], None, None)?;
    }

    let profile_path = format!("{}/profile.txt", profile_dir);

    let file = File::open(profile_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let mod_id = line?;
        Command::new("ferium").arg("add").arg(mod_id).status()?;
    }

    Ok(())
}
