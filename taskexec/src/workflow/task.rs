// This part is used to generate a TaskList based on the user input.

use crate::modules::Module;

// A Task is the interpreted/parsed version of a RawTask.
pub struct TaskList {
    pub list: Vec<Task>,
}

#[derive(Clone)]
pub struct Task {
    // Placeholder
    pub task: String,
    pub module: Module
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            list: Vec::<Task>::new(),
        }
    }
}
