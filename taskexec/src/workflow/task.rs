// This part is used to generate a TaskList based on the user input.
use serde::Deserialize;
use crate::modules::Module;

// A Task is the interpreted/parsed version of a RawTask.

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub name: Option<String>,
    pub module: Module,
    pub action: String,
    pub params: Option<Vec<String>>
}

impl Task {
    pub fn new() -> Task {
        Task {
            name: None,
            module: Module::None,
            action: String::new(),
            params: None
        }
    }

    pub fn from(name: Option<String>, module: Module, action: String, params: Option<Vec<String>>) -> Task {
        Task {
            name,
            module,
            action,
            params
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            list: Vec::<Task>::new(),
        }
    }
    pub fn from(list: Vec<Task>) -> TaskList {
        TaskList {
            list
        }
    }
}
