// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::host::Host;
use crate::workflow::task::TaskList;
use crate::workflow::run::dry_run_task;

#[derive(Debug)]
pub struct Assignment {
    pub runningmode: RunningMode,
    pub host: Host,
    pub tasklist: TaskList,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: Host {address: String::from("")},
            tasklist: TaskList::new(),
        }
    }

    pub fn from(runningmode: RunningMode, host: Host, tasklist: TaskList) -> Assignment {
        Assignment {
            runningmode,
            host,
            tasklist
        }
    }

    pub fn dry_run(&self) -> ChangeList {
        let mut changelist = ChangeList::new();
        for task in self.tasklist.list.iter() {
            let taskdryrunresult = dry_run_task(task.clone());
            changelist.list.push(taskdryrunresult);
        }
        changelist
    }
}

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}
