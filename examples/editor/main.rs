use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use ui_composer::prelude::*;

#[derive(Serialize, Deserialize)]
struct Todos {
    pub project_name: Editable<String>,
    pub todos: EditableList<Todo>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    pub text: String,
    pub done: Editable<bool>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            project_name: Editable::new(String::from("Untitled")),
            todos: EditableList::new(),
        }
    }

    /// Saves to a path!
    pub fn save_to(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::write(
            path,
            toml::to_string_pretty(&self).expect("Failed to save project!"),
        )?;
        Ok(())
    }
}

impl Todo {
    pub fn new(text: String) -> Self {
        Self {
            text,
            done: Editable::new(false),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let todos = Todos::new();
    let mut list = todos.todos.lock_mut();
    list.push_cloned(Todo::new(format!("Do dishes!")));
    list.push_cloned(Todo::new(format!("Say hi to benichi!")));
    drop(list);
    todos.save_to(PathBuf::from("./examples/editor/todos.toml").as_path())?;
    Ok(())
}
