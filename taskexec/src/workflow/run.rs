use crate::modules::apt::dry_run_apt_task;
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

    match task.module {
        Module::Apt => { dry_run_apt_task(task)}
    }

}

pub fn applychange(change: Change) -> TaskResult {
    match change.module {
        Module::Apt => {
            crate::modules::apt::apply_apt_task(change)
        }
    }
}