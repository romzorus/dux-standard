// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::change::ChangeList;
use crate::host::HostList;
use crate::task::TaskList;

pub struct Assignment {
    hostlist: HostList,
    tasklist: TaskList,
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            hostlist: HostList::new(),
            tasklist: TaskList::new(),
        }
    }

    pub fn dry_run(&self) -> ChangeList {
        ChangeList::new()
    }
}
