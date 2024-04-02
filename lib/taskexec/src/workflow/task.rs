// This part is used to generate a TaskList based on the user input.
use serde::Deserialize;
use crate::modules::ModuleBlockExpectedState;
use crate::workflow::change::{ChangeList, ModuleBlockChange, TaskChange};
use connection::prelude::*;


#[derive(Debug, Clone, Deserialize)]
pub struct TaskBlock {
    pub name: Option<String>,
    pub steps: Vec<ModuleBlockExpectedState>,
}

impl TaskBlock {
    pub fn new() -> TaskBlock {
        TaskBlock {
            name: None,
            steps: Vec::new(),
        }
    }

    pub fn from(name: Option<String>, steps: Vec<ModuleBlockExpectedState>) -> TaskBlock {
        TaskBlock {
            name,
            steps
        }   
    }

    pub fn dry_run_task(&self, hosthandler: &mut HostHandler) -> TaskChange {
        let mut list: Vec<ModuleBlockChange> = Vec::new();

        for moduleblock in self.clone().steps.into_iter() {
            let moduleblockchange = moduleblock.dry_run_moduleblock(hosthandler);
            list.push(moduleblockchange);
        }

        if list.iter().all(|x| x.apicalls.is_none()) {
            TaskChange::from(None)
        } else {
            TaskChange::from(Some(list))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<TaskBlock>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: Vec::<TaskBlock>::new(),
        }
    }
    pub fn from(tasks: Vec<TaskBlock>) -> TaskList {
        TaskList {
            tasks
        }
    }
    pub fn dry_run_tasklist(&self, correlationid: String, hosthandler: &mut HostHandler) -> ChangeList {
        let mut list: Vec<TaskChange> = Vec::new();

        for taskcontent in self.tasks.clone().iter() {
            let taskchange = taskcontent.dry_run_task(hosthandler);
            list.push(taskchange);
        }

        if list.iter().all(|x| x.stepchanges.is_none()) {
            ChangeList::from(None, hosthandler.clone())
        } else {
            ChangeList::from(Some(list), hosthandler.clone())
        }
    }
}