use color_eyre::{
    self,
    eyre::{bail, ContextCompat, WrapErr},
};
use ratatui::{backend::Backend, widgets::*};
use ratatui::{widgets::List, Terminal};
use serde::Deserialize;
use serde_yaml::{self, Value};
use std::{collections::BTreeMap, fs, path::Path, time::Duration};

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

/// An item
#[derive(Clone, Debug)]
struct Item {
    key: String,
    desc: Option<Value>,
    summary: Option<Value>,
}

/// Struct that keeps list of available items and current status
#[derive(Clone, Debug)]
struct ItemList {
    list: Vec<Item>,
    state: ListState,
}
impl ItemList {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            state: ListState::default(),
        }
    }

    fn add_item(&mut self, key: String, desc: Option<Value>, summary: Option<Value>) -> Self {
        self.list.push(Item { key, desc, summary });
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

pub fn init_config() -> color_eyre::Result<Vec<String>> {
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

    Ok(items.list.iter().map(|x| x.key.clone()).collect())
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>, conf: Vec<String>, tick_rate: Duration) {}
