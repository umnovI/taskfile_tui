use color_eyre::{
    self,
    eyre::{bail, ContextCompat, WrapErr},
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::{backend::Backend, widgets::*};
use serde::Deserialize;
use serde_yaml::{self, Value};
use std::{
    collections::BTreeMap,
    fs,
    time::{Duration, Instant},
};

use super::tui;
use super::utils;

/// List of allowed filenames in order of importance.
const TASKFILE_NAMES: [&str; 2] = ["Taskfile.yml", "Taskfile.yaml"];
/// Supported Taskfile version.
const TASKFILE_VERSION: &str = "3";

/// Taskfile struct
#[derive(Debug, Deserialize)]
struct Taskfile {
    version: String,
    tasks: BTreeMap<String, BTreeMap<String, Value>>,
}

/// Item properties
#[derive(Clone, Debug)]
pub struct ItemProps {
    pub desc: Option<Value>,
    pub summary: Option<Value>,
}

/// Struct that keeps full list of available items and their properties and current status
#[derive(Clone, Debug)]
pub struct ItemList {
    pub list: BTreeMap<String, ItemProps>,
    pub state: ListState,
}

impl ItemList {
    fn new() -> Self {
        Self {
            list: BTreeMap::new(),
            state: ListState::default(),
        }
    }

    fn add_item(&mut self, name: String, desc: Option<Value>, summary: Option<Value>) -> Self {
        self.list.insert(name, ItemProps { desc, summary });
        Self {
            list: self.list.clone(),
            state: self.state.clone(),
        }
    }

    fn select_first(&mut self) {
        self.state.select(Some(0))
    }

    fn select_next(&mut self) {
        match self.state.selected() {
            Some(i) if i >= self.list.len() - 1 => self.select_first(),
            Some(i) => self.state.select(Some(i + 1)),
            None => self.select_first(),
        }
    }

    fn select_prev(&mut self) {
        match self.state.selected() {
            Some(i) if i == 0 => self.state.select(Some(self.list.len() - 1)),
            Some(i) => self.state.select(Some(i - 1)),
            None => self.select_first(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct App {
    /// List of task names
    pub name_list: Vec<String>,
    /// List of all tasks with corresponding properties
    pub items: ItemList,
}

impl App {
    fn new(name_list: Vec<String>, items: ItemList) -> Self {
        Self { name_list, items }
    }

    /// Get currently selected item's name
    fn get_current(&self) -> Option<&str> {
        match self.items.state.selected() {
            Some(i) => match self.name_list.get(i) {
                Some(name) => Some(name),
                None => None,
            },
            None => None,
        }
    }

    /// Returns description of currently selected item
    pub fn get_desc(&self) -> &str {
        let cur_item = self.get_current();
        match cur_item {
            Some(key) => match &self.items.list[key].desc {
                Some(val) => match Value::as_str(&val) {
                    Some(val) => &val,
                    _ => "Not a string.",
                },
                _ => "Description is empty.",
            },
            _ => "Item not found.",
        }
    }

    /// Returns summary of currently selected item
    pub fn get_summary(&self) -> &str {
        let cur_item = self.get_current();
        match cur_item {
            Some(key) => match &self.items.list[key].summary {
                Some(val) => match Value::as_str(&val) {
                    Some(val) => &val,
                    _ => "Not a string.",
                },
                _ => "Summary is empty.",
            },
            _ => "Item not found.",
        }
    }
}

/// Initialize Taskfile.
///
/// Returns Vec with available task names
pub fn init() -> color_eyre::Result<App> {
    let taskfile = utils::get_filepath(&TASKFILE_NAMES);

    let tasks = {
        let file = fs::read_to_string(taskfile.wrap_err("Could not find Taskfile")?)
            .wrap_err("Could not read found Taskfile")?;
        let data: Taskfile = serde_yaml::from_str(&file).wrap_err("Could not parse Taskfile.")?;
        if data.version != TASKFILE_VERSION {
            bail!("Unsupported Taskfile version. Supported version is {TASKFILE_VERSION}")
        }
        data.tasks
    }; // File only is closed when it gets out of scope.

    // Iterating over tasks and making them into a list
    let mut items = ItemList::new();
    for (taskname, taskdata) in tasks {
        let desc: Option<Value> = if taskdata.contains_key("desc") {
            Some(taskdata["desc"].clone())
        } else {
            None
        };
        let summary: Option<Value> = if taskdata.contains_key("summary") {
            Some(taskdata["summary"].clone())
        } else {
            None
        };
        items.add_item(taskname, desc, summary);
    }

    Ok(App::new(
        items.list.iter().map(|x| x.0.clone()).collect(),
        items,
    ))
}

/// Start main loop
pub fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> color_eyre::Result<()> {
    let mut last_tick = Instant::now();

    // Selecting default value
    app.items.select_first();

    loop {
        terminal.draw(|frame| tui::ui(frame, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Down => app.items.select_next(),
                        KeyCode::Up => app.items.select_prev(),
                        _ => {}
                    }
                }
            }
        }

        // Have no clue what it does
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
