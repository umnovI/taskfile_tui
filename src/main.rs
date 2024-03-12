

use std::time::Duration;

mod app;
mod errors;
mod tui;
mod utils;

fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;

    // Init config file
    let mut app = app::init()?;

    // Init Terminal
    let mut terminal = tui::init()?;
    // Start App
    app::run(&mut terminal, &mut app, Duration::from_millis(250))?;
    // Restore terminal state
    tui::restore()?;

    // Execute commands after restore

    Ok(())
}
