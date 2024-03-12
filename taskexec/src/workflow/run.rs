use crate::modules::Module;
use crate::workflow::change::Change;
use crate::workflow::task::Task;
use crate::workflow::result::TaskResult;

pub fn dry_run_task(task: Task) -> Change {

    match task.module {
        Module::None => { Change::new() }
        Module::Apt => { crate::modules::apt::dry_run_apt_task(task) }
    }
}

pub fn apply_change(change: Change) -> TaskResult {
    match change.module {
        Module::None => { TaskResult::new() }
        Module::Apt => { crate::modules::apt::apply_apt_change(change) }
    }
}