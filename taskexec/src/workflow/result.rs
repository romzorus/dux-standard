// This part is used to define the result structure.

#[derive(Debug)]
pub struct TaskResult {
    pub exitcode: Option<usize>,
    pub stdout: Option<String>,
    pub stderr: Option<String>
}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            exitcode: None,
            stdout: None,
            stderr: None
        }
    }

    pub fn none() -> TaskResult {
        TaskResult {
            exitcode: None,
            stdout: None,
            stderr: None
        }
    }
}

#[derive(Debug)]
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