// This part is used to generate a TaskList based on the user input.

use crate::modules::Module;

// A Task is the interpreted/parsed version of a RawTask.

#[derive(Debug, Clone)]
pub struct Task {
    pub module: Module,
    pub action: String,
    pub params: Option<Vec<String>>
}

impl Task {
    pub fn new() -> Task {
        Task {
            module: Module::None,
            action: String::new(),
            params: None
        }
    }

    pub fn from(module: Module, action: String, params: Option<Vec<String>>) -> Task {
        Task {
            module,
            action,
            params
        }
    }
}

#[derive(Debug)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            list: Vec::<Task>::new(),
        }
    }
}
