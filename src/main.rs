use color_eyre::{
    self,
    eyre::{bail, ContextCompat, WrapErr},
};
use ratatui::widgets::List;
use serde::Deserialize;
use serde_yaml::{self, Value};
use std::{collections::BTreeMap, fs, path::Path, time::Duration};

mod app;
mod errors;
mod tui;
mod utils;

fn main() -> color_eyre::Result<()> {
    errors::install_hooks()?;

    // Init config file
    let conf = app::init_config()?;
    // Init Tick rate
    let tick_rate = Duration::from_millis(250);
    // Init Terminal
    let mut terminal = tui::init()?;
    // Start App
    app::run(&mut terminal, conf, tick_rate)?;
    // let tui_list = List::new(conf);

    // Restore terminal state
    tui::restore()?;
    Ok(())
}
