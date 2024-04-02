// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::{TaskResult, TaskListResult};
use crate::workflow::result::ModuleBlockResult;
use crate::modules::ModuleApiCall;
use connection::prelude::*;

use super::result::ApiCallResult;


#[derive(Debug, Clone)]
pub struct ModuleBlockChange {
    pub apicalls: Option<Vec<ModuleApiCall>>
}

impl ModuleBlockChange {
    pub fn new() -> ModuleBlockChange {
        ModuleBlockChange {
            apicalls: Some(Vec::new())
        }
    }

    pub fn none() -> ModuleBlockChange {
        ModuleBlockChange {
            apicalls: None
        }
    }

    pub fn from(apicalls: Option<Vec<ModuleApiCall>>) -> ModuleBlockChange {
        ModuleBlockChange {
            apicalls
        }
    }

    pub fn apply_moduleblockchange(&self, hosthandler: &mut HostHandler) -> ModuleBlockResult {
        match self.apicalls.clone() {
            Some(moduleapicalllist) => {
                let mut results: Vec<ApiCallResult> = Vec::new();
                for moduleapicall in moduleapicalllist {
                    let apicallresult = match moduleapicall {
                        ModuleApiCall::None => { ApiCallResult::none() }
                        ModuleApiCall::Apt(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::YumDnf(block) => { block.apply_moduleblock_change(hosthandler) }
                    };
                    results.push(apicallresult);
                }
                return ModuleBlockResult::from(Some(results));
            }
            None => { ModuleBlockResult::none() }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskChange {
    pub stepchanges: Option<Vec<ModuleBlockChange>>
}

impl TaskChange {
    pub fn new() -> TaskChange {
        TaskChange {
            stepchanges: Some(Vec::new())
        }
    }

    pub fn none() -> TaskChange {
        TaskChange {
            stepchanges: None
        }
    }

    pub fn from(stepchanges: Option<Vec<ModuleBlockChange>>) -> TaskChange {
        TaskChange {
            stepchanges
        }
    }

    pub fn apply_taskchange(&self, hosthandler: &mut HostHandler) -> TaskResult {
        match self.stepchanges.clone() {
            None => {
                return TaskResult::none();
            }
            Some(moduleblockchangelist) => {
                let mut stepresults: Vec<ModuleBlockResult> = Vec::new();

                for moduleblockchange in moduleblockchangelist.iter() {
                    let moduleblockresultlist = moduleblockchange.apply_moduleblockchange(hosthandler);
                    stepresults.push(moduleblockresultlist);
                }

                return TaskResult::from(Some(stepresults));
            }
        }
    }
}

#[derive(Clone)]
pub struct ChangeList {
    pub taskchanges: Option<Vec<TaskChange>>,
    hosthandler: HostHandler,
}

impl ChangeList {
    pub fn new() -> ChangeList {
        ChangeList {
            taskchanges: Some(Vec::new()),
            hosthandler: HostHandler::new(),
        }
    }

    pub fn from(taskchanges: Option<Vec<TaskChange>>, hosthandler: HostHandler) -> ChangeList {
        ChangeList {
            taskchanges,
            hosthandler,
        }
    }

    pub fn apply_changelist(&mut self, hosthandler: &mut HostHandler) -> TaskListResult {

        match &self.taskchanges {
            None => { return TaskListResult::none(); }
            Some(taskchangelist) => {

                let mut tasklistresult = TaskListResult::new();

                for taskchange in taskchangelist.iter() {
                    tasklistresult.taskresults.push(
                      taskchange.apply_taskchange(hosthandler)  
                    );
                }
        
                return tasklistresult;
            }
        }
    }
}
