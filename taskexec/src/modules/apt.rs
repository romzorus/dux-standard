// APT Module : handle packages in Debian-like distributions

// A package must handle
//  - mode Apply ChangeList -> ExecResult
//  - mode DryRun TaskList -> ChangeList
//  - action_description : HashMap

use crate::workflow::change::Change;
use crate::workflow::result::TaskResult;
use crate::workflow::task::Task;
use crate::modules::Module;

pub struct AptTask {
    package: String,
    action: String,
}

pub fn dry_run_apt_task(task: Task) -> Change {
    assert_eq!(task.module, Module::Apt);
    match task.action.as_str() {
        "install" => {
            // First, check if the package is already installed.
            // If already installed, just return a Change::none()
            Change {
                module: crate::modules::Module::Apt,
                action: Some(String::from("install")),
                params: Some(task.params.unwrap())
            }
        }
        _ => { Change::none() }
    }
}

pub fn apply_apt_change(change: Change) -> TaskResult {
    assert_eq!(change.module, Module::Apt);
    match change.action.unwrap().as_str() {
        "install" => {
            // Install package
            println!("****** Install package : {:?}", change.params.unwrap());
            TaskResult {
                exitcode: Some(0),
                stdout: Some(String::from("Installation successfull")),
                stderr: None
            }
        }
        _ => { TaskResult::none() }
    }
}