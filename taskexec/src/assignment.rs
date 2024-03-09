// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::change::ChangeList;
use crate::host::Host;
use crate::task::TaskList;

pub struct Assignment {
    host: Host,
    tasklist: TaskList,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            host: Host {address: String::from("")},
            tasklist: TaskList::new(),
        }
    }

    pub fn dry_run(&self) -> ChangeList {
        ChangeList::new()
    }
}
