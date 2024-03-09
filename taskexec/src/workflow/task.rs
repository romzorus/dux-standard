// This part is used to generate a TaskList based on the user input.

// A Task is the interpreted/parsed version of a RawTask.
pub struct TaskList {
    tasklist: Vec<Task>,
}

pub struct Task {
    task: String,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasklist: Vec::<Task>::new(),
        }
    }
}
