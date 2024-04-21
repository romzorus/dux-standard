// This part is used to define the result structure.

#[derive(Debug, Clone)]
pub struct ApiCallResult {
    pub exitcode: Option<i32>,
    pub output: Option<String>,
    pub status: ApiCallStatus
}

impl ApiCallResult {
    pub fn new() -> ApiCallResult {
        ApiCallResult {
            exitcode: None,
            output: None,
            status: ApiCallStatus::Unset
        }
    }

    pub fn none() -> ApiCallResult {
        ApiCallResult {
            exitcode: None,
            output: None,
            status: ApiCallStatus::None
        }
    }

    pub fn from(exitcode: Option<i32>, output: Option<String>, status: ApiCallStatus) -> ApiCallResult {
        ApiCallResult {
            exitcode,
            output,
            status
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleBlockResult {
    pub apicallresults: Vec<ApiCallResult>
}

impl ModuleBlockResult {
    pub fn new() -> ModuleBlockResult {
        ModuleBlockResult {
            apicallresults: Vec::new()
        }
    }

    pub fn none() -> ModuleBlockResult {
        ModuleBlockResult {
            apicallresults: Vec::from([ApiCallResult::none()])
        }
    }

    pub fn from(apicallresults: Vec<ApiCallResult>) -> ModuleBlockResult {
        ModuleBlockResult {
            apicallresults
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskResult {
    pub stepresults: Option<Vec<ModuleBlockResult>>
}

impl TaskResult {
    pub fn new() -> TaskResult {
        TaskResult {
            stepresults: Some(Vec::new())
        }
    }

    pub fn none() -> TaskResult {
        TaskResult {
            stepresults: None
        }
    }

    pub fn from(stepresults: Option<Vec<ModuleBlockResult>>) -> TaskResult {
        TaskResult {
            stepresults
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskListResult {
    pub taskresults: Vec<TaskResult>,
}

impl TaskListResult {
    pub fn new() -> TaskListResult {
        TaskListResult {
            taskresults: Vec::new(),
        }
    }

    // The 'results' field could be turned into an Option but this complexifies the apply_changelist() method
    // in change.rs (we need to deconstruct...etc). For now, results = 'None' is just an empty vector.
    // TODO : turn 'results' into an Option<Vec<TaskResult>>.
    pub fn none() -> TaskListResult {

        // TODO : set all blockmatrix results to None as well
        TaskListResult {
            taskresults: Vec::new(),

        }
    }

    pub fn from(taskresults: Vec<TaskResult>) -> TaskListResult {
        TaskListResult {
            taskresults,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ApiCallStatus {
    Unset,
    None,
    ChangeSuccessful(String),
    Failure(String),
    AllowedFailure(String)
}
