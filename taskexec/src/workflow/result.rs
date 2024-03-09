// This part is used to define the result structure.

pub struct TaskResult {
    pub exitcode: usize,
    pub stdout: String,
    pub stderr: String,
}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            exitcode: 0,
            stdout: String::from(""),
            stderr: String::from(""),
        }
    }
}

pub struct TaskListResult {
    pub results: Vec<TaskResult>
}

impl TaskListResult {
    pub fn new() -> TaskListResult {
        TaskListResult {
            results: Vec::<TaskResult>::new()
        }
    }
}