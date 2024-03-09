// APT Module : handle packages in Debian-like distributions

// A package must handle
//  - mode Apply ChangeList -> ExecResult
//  - mode DryRun TaskList -> ChangeList
//  - action_description : HashMap

use crate::workflow::change::{Change, ChangeList};
use crate::workflow::result::TaskResult;

pub struct AptTask {
    package: String,
    action: String,
}

pub fn dry_run_apt_task() -> ChangeList {
    // Placeholder
    ChangeList::new()
}

pub fn apply_apt_task(change: Change) -> TaskResult {
    // Placeholder
    TaskResult::new()
}