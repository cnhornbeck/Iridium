// src/profile/profile.rs

#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub mods: Vec<String>,
}

impl Profile {
    pub fn new(name: String, mods: Vec<String>) -> Profile {
        Profile { name, mods }
    }
}
