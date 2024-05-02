// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::{ModuleBlockResult, TaskResult, TaskListResult, ApiCallResult, ApiCallStatus};
use crate::modules::ModuleApiCall;
use crate::modules::{DryRun, Apply};
use connection::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleBlockChange {
    AllowedFailure(String),
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

    pub fn display(&self) -> Vec<String> {

        match self {
            ModuleBlockChange::AllowedFailure(message) => { return Vec::from([message.clone()]); }
            ModuleBlockChange::AlreadyMatched(message) => { return Vec::from([message.clone()]); }
            ModuleBlockChange::FailedToEvaluate(message) => { return Vec::from([message.clone()]); }
            ModuleBlockChange::ModuleApiCalls(changeslist) => {
                let mut display_contents: Vec<String> = Vec::new();
                for change in changeslist {
                    let apicalldisplay = match change {
                        ModuleApiCall::None(message) => { message.clone() }
// **BEACON_1**
                        ModuleApiCall::LineInFile(block) => { block.display() }
                        ModuleApiCall::Command(block) => { block.display() }
                        ModuleApiCall::Apt(block) => { block.display() }
                        ModuleApiCall::Ping(block) => { block.display() }
                        ModuleApiCall::YumDnf(block) => { block.display() }
                    };
                    display_contents.push(apicalldisplay);
                }
                return display_contents;
            }
        }
    }

    pub fn apply_moduleblockchange(&self, hosthandler: &mut HostHandler) -> ModuleBlockResult {

        match self {
            ModuleBlockChange::AllowedFailure(_message) => { return ModuleBlockResult::none() }
            ModuleBlockChange::AlreadyMatched(_message) => { return ModuleBlockResult::none() }
            ModuleBlockChange::FailedToEvaluate(_message) => { return ModuleBlockResult::none() }
            ModuleBlockChange::ModuleApiCalls(changeslist) => {
                let mut results: Vec<ApiCallResult> = Vec::new();
                for change in changeslist {
                    let apicallresult = match change {
                        ModuleApiCall::None(_) => { ApiCallResult::none() }
// **BEACON_2**
                        ModuleApiCall::LineInFile(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::Command(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::Apt(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::Ping(block) => { block.apply_moduleblock_change(hosthandler) }
                        ModuleApiCall::YumDnf(block) => { block.apply_moduleblock_change(hosthandler) }
                    };                   
                    results.push(apicallresult);
                }
                return ModuleBlockResult::from(results);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskChange {
    pub stepchanges: Vec<ModuleBlockChange>,
    pub allowed_failures: Vec<bool>
}

impl TaskChange {
    pub fn new() -> TaskChange {
        TaskChange {
            stepchanges: Vec::new(),
            allowed_failures: Vec::new()
        }
    }

    pub fn from(stepchanges: Vec<ModuleBlockChange>, allowed_failures: Vec<bool>) -> TaskChange {
        TaskChange {
            stepchanges,
            allowed_failures
        }
    }

    pub fn apply_taskchange(&self, hosthandler: &mut HostHandler) -> TaskResult {

        let mut stepresults: Vec<ModuleBlockResult> = Vec::new();

        for (mbindex, moduleblockchange) in self.stepchanges.iter().enumerate() {
            let mut moduleblockresultlist = moduleblockchange.apply_moduleblockchange(hosthandler);

            // Change Failures into AllowedFailures before pushing to stepresults
            // It is done at this level and not at module level so modules don't have to bother with upper level logic.
            // We just want modules to return Failures when they fail, nothing more.
            if self.allowed_failures[mbindex] {
                for (index, apicallresult) in moduleblockresultlist.apicallresults.clone().iter().enumerate() {
                    if let ApiCallStatus::Failure(message) = &apicallresult.status {
                        moduleblockresultlist.apicallresults[index].status = ApiCallStatus::AllowedFailure(message.to_string());
                    }
                }
                stepresults.push(moduleblockresultlist.clone());
            } else {
                stepresults.push(moduleblockresultlist.clone());
                // If a failure is encountered in a step, stop the "apply" there.
            if ! self.allowed_failures[mbindex] {
                for apicallresult in moduleblockresultlist.apicallresults.into_iter() {
                    if let ApiCallStatus::Failure(_) = apicallresult.status {
                        return TaskResult::from(Some(stepresults));
                    }
                }
            }
            }


        }
        return TaskResult::from(Some(stepresults));
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeList {
    pub taskchanges: Option<Vec<TaskChange>>,
    // hosthandler: HostHandler,
}

impl ChangeList {
    pub fn new() -> ChangeList {
        ChangeList {
            taskchanges: Some(Vec::new()),
            // hosthandler: HostHandler::new(),
        }
    }

    pub fn from(taskchanges: Option<Vec<TaskChange>>, hosthandler: HostHandler) -> ChangeList {
        ChangeList {
            taskchanges,
            // hosthandler,
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
