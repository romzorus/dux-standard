// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::{ModuleBlockResult, TaskResult, TaskListResult, ApiCallResult};
use crate::modules::ModuleApiCall;
use connection::prelude::*;

#[derive(Debug, Clone)]
pub enum ModuleBlockChange {
    AlreadyMatched(String),
    FailedToEvaluate(String), // The module can't work on this host (trying to use yum/dnf on Debian for example)
    ModuleApiCalls(Vec<ModuleApiCall>)
}

impl ModuleBlockChange {

    pub fn matched(message: &str) -> ModuleBlockChange {
        ModuleBlockChange::AlreadyMatched(message.to_string())
    }

    pub fn failed_to_evaluate(message: &str) -> ModuleBlockChange {
        ModuleBlockChange::FailedToEvaluate(message.to_string())
    }

    pub fn changes(changes: Vec<ModuleApiCall>) -> ModuleBlockChange {
        ModuleBlockChange::ModuleApiCalls(changes)
    }

    pub fn apply_moduleblockchange(&self, hosthandler: &mut HostHandler) -> ModuleBlockResult {

        match self {
            ModuleBlockChange::AlreadyMatched(message) => { ModuleBlockResult::none() }
            ModuleBlockChange::FailedToEvaluate(message) => { ModuleBlockResult::none() }
            ModuleBlockChange::ModuleApiCalls(changeslist) => {
                let mut results: Vec<ApiCallResult> = Vec::new();
                for change in changeslist {
                    let apicallresult = match change {
                        ModuleApiCall::Apt(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::YumDnf(block) => { block.apply_moduleblock_change(hosthandler) }
                    };
                    results.push(apicallresult);
                }
                return ModuleBlockResult::from(results);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskChange {
    pub stepchanges: Vec<ModuleBlockChange>
}

impl TaskChange {
    pub fn new() -> TaskChange {
        TaskChange {
            stepchanges: Vec::new()
        }
    }

    pub fn from(stepchanges: Vec<ModuleBlockChange>) -> TaskChange {
        TaskChange {
            stepchanges
        }
    }

    pub fn apply_taskchange(&self, hosthandler: &mut HostHandler) -> TaskResult {

        let mut stepresults: Vec<ModuleBlockResult> = Vec::new();

        for moduleblockchange in self.stepchanges.iter() {
            let moduleblockresultlist = moduleblockchange.apply_moduleblockchange(hosthandler);
            stepresults.push(moduleblockresultlist);
        }

        return TaskResult::from(Some(stepresults));
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