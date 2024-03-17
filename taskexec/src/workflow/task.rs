// This part is used to generate a TaskList based on the user input.
use serde::Deserialize;
use crate::modules::ModuleBlock;
use crate::workflow::change::{ChangeList, ModuleBlockChange, TaskChange};


#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub name: Option<String>,
    pub tasks: Vec<ModuleBlock>,
}

impl Task {
    pub fn new() -> Task {
        Task {
            name: None,
            tasks: Vec::new(),
        }
    }

    pub fn from(name: Option<String>, tasks: Vec<ModuleBlock>) -> Task {
        Task {
            name,
            tasks,
        }
    }

    pub fn dry_run_task(&self) -> TaskChange {
        let mut list: Vec<ModuleBlockChange> = Vec::new();

        for moduleblock in self.clone().tasks.into_iter() {
            let moduleblockchange = moduleblock.dry_run_moduleblock();
            list.push(moduleblockchange);
        }

        if list.iter().all(|x| x.module.is_none()) {
            TaskChange::from(None)
        } else {
            TaskChange::from(Some(list))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            list: Vec::<Task>::new(),
        }
    }
    pub fn from(list: Vec<Task>) -> TaskList {
        TaskList {
            list
        }
    }
    pub fn dry_run_tasklist(&self, correlationid: String) -> ChangeList {
        let mut list: Vec<TaskChange> = Vec::new();

        for task in self.list.clone().into_iter() {
            let taskchange = task.dry_run_task();
            list.push(taskchange);
        }

        if list.iter().all(|x| x.list.is_none()) {
            ChangeList::from(correlationid, None)
        } else {
            ChangeList::from(correlationid, Some(list))
        }
    }
}