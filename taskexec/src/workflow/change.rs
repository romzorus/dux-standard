// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::{TaskResult, TaskListResult};
use crate::workflow::result::ModuleBlockResult;
use crate::modules::ModuleBlock;


#[derive(Debug, Clone)]
pub struct ModuleBlockChange {
    pub module: Option<ModuleBlock>
}

impl ModuleBlockChange {
    pub fn new_none() -> ModuleBlockChange {
        ModuleBlockChange {
            module: None
        }
    }

    pub fn apply_moduleblockchange(&self) -> ModuleBlockResult {
        match self.module.clone().unwrap() {
            ModuleBlock::None => {ModuleBlockResult::new_none() }
            ModuleBlock::Apt(block) => { block.apply_moduleblock_change() }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskChange {
    pub list: Option<Vec<ModuleBlockChange>>
}

impl TaskChange {
    pub fn new() -> TaskChange {
        TaskChange {
            list: Some(Vec::new())
        }
    }

    pub fn none() -> TaskChange {
        TaskChange {
            list: None
        }
    }

    pub fn from(list: Option<Vec<ModuleBlockChange>>) -> TaskChange {
        TaskChange {
            list
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangeList {
    pub correlationid: String,
    pub list: Option<Vec<TaskChange>>
}

impl ChangeList {
    pub fn new(correlationid: String) -> ChangeList {
        ChangeList {
            correlationid,
            list: Some(Vec::new()),
        }
    }

    pub fn from(correlationid: String, list: Option<Vec<TaskChange>>) -> ChangeList {
        ChangeList {
            correlationid,
            list,
        }
    }

    pub fn apply_changelist(&self) -> TaskListResult {

        match self.list {
            None => { TaskListResult::from(self.correlationid.clone(), vec![])}
            Some(_) => {
                let mut tasklistresult = TaskListResult::new(self.correlationid.clone());

                for taskchange in self.list.clone().unwrap().clone().into_iter() {

                    match taskchange.list {
                        None => {
                            tasklistresult.results.push(TaskResult { list: None });
                        }
                        Some(_) => {
                            let mut list: Vec<ModuleBlockResult> = Vec::new();
        
                            for moduleblockchange in taskchange.list.unwrap().clone().into_iter() {
                                let moduleblockresult = moduleblockchange.apply_moduleblockchange();
                                list.push(moduleblockresult);
                            }
                
                            tasklistresult.results.push(TaskResult { list: Some(list) });
                        }
                    }

                }
        
                tasklistresult
            }
        }
    }
}
