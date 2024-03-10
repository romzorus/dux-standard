// APT Module : handle packages in Debian-like distributions

// A package must handle
//  - mode Apply ChangeList -> ExecResult
//  - mode DryRun TaskList -> ChangeList
//  - action_description : HashMap

use crate::workflow::change::{Change, ChangeList};
use crate::workflow::result::TaskResult;
use crate::workflow::task::Task;

pub struct AptTask {
    package: String,
    action: String,
}

pub fn dry_run_apt_task(task: Task) -> Change {
    // Placeholder
    Change {
        module: crate::modules::Module::Apt,
        action: String::from(""),
        parameters: vec![]
    }
}

pub fn apply_apt_change(change: Change) -> TaskResult {
    // Placeholder
    TaskResult::new()
}