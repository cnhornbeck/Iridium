use std::error::Error;

mod app;

mod crossterm;

mod ui;

mod profile_manager;

mod github_integration;

fn main() -> Result<(), Box<dyn Error>> {
    crate::crossterm::run()?;

    Ok(())
}
