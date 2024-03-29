// This part is used to define the result structure.

#[derive(Debug, Clone)]
pub struct ModuleBlockResult {
    pub exitcode: Option<usize>,
    pub stdout: Option<String>,
    pub stderr: Option<String>
}

impl ModuleBlockResult {
    pub fn new_none() -> ModuleBlockResult {
        ModuleBlockResult {
            exitcode: None,
            stdout: None,
            stderr: None
        }
    }

    pub fn from(exitcode: Option<usize>, stdout: Option<String>, stderr: Option<String>) -> ModuleBlockResult {
        ModuleBlockResult {
            exitcode,
            stdout,
            stderr
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub list: Option<Vec<ModuleBlockResult>>
}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            list: Some(Vec::new())
        }
    }

    pub fn none() -> TaskResult {
        TaskResult {
            list: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskListResult {
    pub correlationid: String,
    pub results: Vec<TaskResult>
}

impl TaskListResult {
    pub fn new(correlationid: String) -> TaskListResult {
        TaskListResult {
            correlationid,
            results: Vec::<TaskResult>::new()
        }
    }

    pub fn from(correlationid: String, results: Vec<TaskResult>) -> TaskListResult {
        TaskListResult {
            correlationid,
            results
        }
    }
}