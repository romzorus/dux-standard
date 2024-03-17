// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::host::Host;
use crate::workflow::task::TaskList;
use crate::workflow::result::TaskListResult;


#[derive(Debug)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: Host,
    pub tasklist: TaskList,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            correlationid: new_correlationid(),
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: Host {address: String::from("")},
            tasklist: TaskList::new(),
        }
    }

    pub fn from(
        runningmode: RunningMode,
        host: Host,
        tasklist: TaskList ) -> Assignment {
        Assignment {
            correlationid: new_correlationid(),
            runningmode,
            host,
            tasklist
        }
    }

    pub fn dry_run(&self) -> ChangeList {

        self.tasklist.dry_run_tasklist(self.correlationid.clone())
        
    }

    pub fn apply(&self) -> TaskListResult {

        TaskListResult::new(self.correlationid.clone())

    }
}

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}

pub fn new_correlationid() -> String {
    // Placeholder
    String::from("abcd")
}