use std::io::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TodoItem {
    pub completed: bool,
    pub name: String,
    pub important: bool,
}

impl TodoItem {
    pub fn new(name: &str) -> Self {
        TodoItem {
            completed: false,
            name: name.to_string(),
            important: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todo_items: Vec<TodoItem>,
    pub name: String,
    pub completed: bool,
    pub important: bool,
}

impl TodoList {
    pub fn new(name: &str) -> Self {
        TodoList {
            todo_items: Vec::new(),
            name: name.to_string(),
            completed: false,
            important: false,
        }
    }

    pub fn remove(&mut self, todo_item: TodoItem) {
        self.todo_items.retain(|item| item != &todo_item);
    }

    pub fn check_completed(&self) -> bool {
        self.todo_items.iter().all(|item| item.completed)
    }

    fn serialize(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&self)?)
    }

    fn deserialize(json_string: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json_string)?)
    }
}
