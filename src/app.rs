use anyhow::{bail, Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::{backend::Backend, widgets::*};
use serde::Deserialize;
use serde_yaml::{self, Value};
use std::{collections::BTreeMap, fs, process::Command, time::Duration};

use crate::Args;

use crate::tui;
use crate::utils;

/// List of allowed filenames in order of importance.
const TASKFILE_NAMES: [&str; 2] = ["Taskfile.yml", "Taskfile.yaml"];
/// Supported Taskfile version.
const TASKFILE_VERSION: &str = "3";

/// Taskfile struct
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Deserialize)]
struct Taskfile {
    version: String,
    tasks: BTreeMap<String, BTreeMap<String, Value>>,
}

/// Item properties
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct ItemProps {
    pub desc: Option<Value>,
    pub summary: Option<Value>,
}

/// Struct that keeps full list of available items and their properties and current status
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
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
            Some(0) => self.state.select(Some(self.list.len() - 1)),
            Some(i) => self.state.select(Some(i - 1)),
            None => self.select_first(),
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone)]
pub struct App {
    /// List of task names
    pub name_list: Vec<String>,
    /// List of all tasks with corresponding properties
    pub items: ItemList,
    /// Whether a selected command should be executed
    pub execute_selected: bool,
}

impl App {
    fn new(name_list: Vec<String>, items: ItemList) -> Self {
        Self {
            name_list,
            items,
            execute_selected: false,
        }
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
                Some(val) => match Value::as_str(val) {
                    Some(val) => val,
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
                Some(val) => match Value::as_str(val) {
                    Some(val) => val,
                    _ => "Not a string.",
                },
                _ => "Summary is empty.",
            },
            _ => "Item not found.",
        }
    }
}

/// Initialize App.
///
/// Returns App object
pub fn init(args: &Args) -> Result<App> {
    let taskfile = utils::get_filepath(args, &TASKFILE_NAMES)?;

    let tasks = {
        let file = fs::read_to_string(taskfile.with_context(|| "Could not find Taskfile")?)
            .with_context(|| "Could not read found Taskfile")?;
        let data: Taskfile =
            serde_yaml::from_str(&file).with_context(|| "Could not parse Taskfile.")?;
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

    if items.list.is_empty() {
        bail!("No tasks found")
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
) -> Result<()> {
    // Selecting default value
    app.items.select_first();

    loop {
        terminal.draw(|frame| tui::ui(frame, app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Down => app.items.select_next(),
                        KeyCode::Up => app.items.select_prev(),
                        KeyCode::Enter => {
                            app.execute_selected = true;
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

/// Start task
pub fn task_exec(args: &Args, app: &App) -> Result<()> {
    let mut exec_args = String::new();
    if args.global {
        exec_args.push_str("-g");
    }

    if let Some(taskname) = app.get_current() {
        if cfg!(target_os = "windows") {
            Command::new("nu")
                .args(["--commands", &format!("task {} {}", taskname, exec_args)])
                .status()?
        } else {
            Command::new("sh")
                .args(["-c", &format!("task {} {}", taskname, exec_args)])
                .status()?
        };
    }

    Ok(())
}
