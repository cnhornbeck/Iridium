// src/profile/profile.rs
pub struct Profile {
    pub name: String,
    pub mods: Vec<String>,
}

impl Profile {
    pub fn new(name: String, mods: Vec<String>) -> Profile {
        Profile { name, mods }
    }

    pub fn add_mod(&mut self, mod_name: String) {
        self.mods.push(mod_name);
    }
}
