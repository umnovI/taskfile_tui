use anyhow::Result;
use clap::Parser;
use std::time::Duration;

mod app;
mod errors;
mod tui;
mod utils;

/// TUI for your Taskfile
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Look for global taskfile
    #[arg(short, long)]
    global: bool,
}

fn main() -> Result<()> {
    errors::install_hooks()?;
    let args = Args::parse();

    // Init app object
    let mut app = app::init(&args)?;

    // Init Terminal
    let mut terminal = tui::init()?;
    // Start App
    app::run(&mut terminal, &mut app, Duration::from_millis(250))?;
    // Restore terminal state
    tui::restore()?;

    // Execute commands after restore
    if app.execute_selected {
        app::task_exec(&args, &app)?;
    }

    Ok(())
}
