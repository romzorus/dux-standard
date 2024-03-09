use crate::modules::Module;
use crate::workflow::change::Change;
use crate::workflow::task::Task;
use crate::workflow::result::TaskResult;

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}

pub fn dry_run_task(task: Task) -> Change {
    // Placeholder
    Change {
        module: Module::Apt,
        action: String::from(""),
        parameters: vec![]
    }
}

pub fn applychange(change: Change) -> TaskResult {
    match change.module {
        Module::Apt => {
            crate::modules::apt::apply_apt_task(change)
        }
    }
}