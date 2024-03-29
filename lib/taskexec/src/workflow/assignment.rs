// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::task::TaskList;
use crate::workflow::result::TaskListResult;
use connection::prelude::*;

#[derive(Debug)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String,
    pub tasklist: TaskList,
    pub hosthandler: HostHandler
}

impl Assignment {
    pub fn new(correlationid: String) -> Assignment {
        Assignment {
            correlationid,
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: String::from(""),
            tasklist: TaskList::new(),
        }
    }

    pub fn from(
        correlationid: String,
        runningmode: RunningMode,
        host: String,
        tasklist: TaskList ) -> Assignment {
        Assignment {
            correlationid,
            runningmode,
            host,
            tasklist
        }
    }

    pub fn dry_run(&self) -> ChangeList {

        self.tasklist.dry_run_tasklist(self.correlationid.clone(), self.host.clone())
        
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