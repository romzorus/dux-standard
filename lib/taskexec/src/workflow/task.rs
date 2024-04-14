// This part is used to generate a TaskList based on the user input.
use serde::Deserialize;
use crate::modules::ModuleBlockExpectedState;
use crate::workflow::change::{ChangeList, ModuleBlockChange, TaskChange};
use connection::prelude::*;

// TODO : ModuleBlockExpectedState need to be wrapped in a 'Step' type
// 1 Step =
//      - 1 ModuleBlockExpectedState
//      - parameters such as :
//          - name
//          - privilege escalation for the ModuleBlock (with_sudo, run_as...etc)
//          - pre/post logic
#[derive(Debug, Clone, Deserialize)]
pub struct TaskBlock {
    pub name: Option<String>,
    pub steps: Vec<ModuleBlockExpectedState>,
    pub with_sudo: Option<bool>
}

impl TaskBlock {
    pub fn new() -> TaskBlock {
        TaskBlock {
            name: None,
            steps: Vec::new(),
            with_sudo: None
        }
    }

    pub fn from(name: Option<String>, steps: Vec<ModuleBlockExpectedState>, with_sudo: Option<bool>) -> TaskBlock {
        TaskBlock {
            name,
            steps,
            with_sudo
        }   
    }

    pub fn dry_run_task(&self, hosthandler: &mut HostHandler) -> TaskChange {
        let mut list: Vec<ModuleBlockChange> = Vec::new();

        for moduleblock in self.clone().steps.into_iter() {
            let moduleblockchange = moduleblock.dry_run_moduleblock(hosthandler);
            list.push(moduleblockchange);
        }

        return TaskChange::from(list)
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

        return ChangeList::from(Some(list), hosthandler.clone());

        // if list.iter().all(|x| x.stepchanges.is_none()) {
        //     ChangeList::from(None, hosthandler.clone())
        // } else {
        //     ChangeList::from(Some(list), hosthandler.clone())
        // }
    }
}