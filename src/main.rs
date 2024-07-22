use std::error::Error;

mod app;

mod crossterm;

mod ui;

mod mod_operations;

fn main() -> Result<(), Box<dyn Error>> {
    crate::crossterm::run()?;

    Ok(())
}
