// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::host::Host;
use crate::workflow::task::TaskList;

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
        ChangeList::new()
    }
}

enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}
