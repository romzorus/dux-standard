// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::host::Host;
use crate::workflow::task::TaskList;
use crate::workflow::run::RunningMode;
use crate::workflow::run::dry_run_task;

pub struct Assignment {
    runningmode: RunningMode,
    host: Host,
    tasklist: TaskList,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            runningmode: RunningMode::DryRun,       // Default behavior = check only
            host: Host {address: String::from("")},
            tasklist: TaskList::new(),
        }
    }

    pub fn dry_run(&self) -> ChangeList {
        assert_eq!(self.runningmode, RunningMode::DryRun);
        let mut changelist = ChangeList::new();
        for task in self.tasklist.list.iter() {
            let taskdryrunresult = dry_run_task(task.clone());
            changelist.list.push(taskdryrunresult);
        }
        changelist
    }
}
