// This part is used to generate a TaskList based on the user input.
use serde::{Deserialize, Deserializer};
use crate::modules::ModuleBlockExpectedState;
use crate::modules::blocks::*;
use crate::workflow::change::{ChangeList, ModuleBlockChange, TaskChange};
use connection::prelude::*;
use errors::Error;


#[derive(Debug, Clone, Deserialize)]
pub struct Step {
    pub name: Option<String>,
    pub run_as: Option<String>,
    pub with_sudo: Option<bool>,
    pub allowed_to_fail: Option<bool>,
    // pub prelogic -> TODO
    // pub postlogic -> TODO

    // This attribute is skipped by serde because it is not defined by the user in the TaskList. It is filled by the
    // .parsemodule() method based on the rest of the attributes (one per module). After applying this method, the
    // moduleblock attribute holds the Expected State ready to be used by the rest of the workflow.
    #[serde(skip)]
    moduleblock: Option<ModuleBlockExpectedState>,

    // FIXME: Having an attribute per module is at the moment the only way found to be able to write "apt:" and not "!apt".
    // It needs a parsemodule() method to check that only one attribute per step is filled.
// **BEACON_1**
    pub command: Option<CommandBlockExpectedState>,
    pub apt: Option<AptBlockExpectedState>,
    pub dnf: Option<YumDnfBlockExpectedState>,
    #[serde(default, deserialize_with = "deserialize_argumentlessmodule")]
    pub ping: Option<Option<PingBlockExpectedState>>, // Double wrapping in order to have Serde distinguish between missing field and None value
    pub yum: Option<YumDnfBlockExpectedState>
}

impl Step {
    pub fn parsemodule(&mut self) -> Result<(), String> {
        let mut counter: u32 = 0; // Used to check that only one module is used per Step

// **BEACON_2**
        if let Some(content) = self.command.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::Command(content)); }
        if let Some(content) = self.apt.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::Apt(content)); }
        if let Some(content) = self.dnf.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::Dnf(content)); }
        if let Some(_content) = self.ping.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::Ping(PingBlockExpectedState{})); } // Ping "content" is always null at the moment
        if let Some(content) = self.yum.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::Yum(content)); }

        if counter > 1 { return Err(String::from("Too much modules defined in this step")); }
        else { return Ok(()); }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskBlock {
    pub name: Option<String>,
    pub steps: Vec<Step>,
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

    pub fn from(name: Option<String>, steps: Vec<Step>, with_sudo: Option<bool>) -> TaskBlock {
        TaskBlock {
            name,
            steps,
            with_sudo
        }   
    }

    pub fn dry_run_task(&self, hosthandler: &mut HostHandler) -> Result<TaskChange, Error> {
        let mut mbchangeslist: Vec<ModuleBlockChange> = Vec::new();
        let mut allowed_failures: Vec<bool> = Vec::new();


        // TODO : add some checking (with_sudo and run_as need to be mutually exclusive)
        for step in self.clone().steps.into_iter() {
            let privilege = match step.with_sudo {
                None => {
                    match step.run_as {
                        None => { Privilege::Usual }
                        Some(username) => { Privilege::AsUser(username) }
                    }
                }
                Some(value) => {
                    if value { Privilege::WithSudo }
                    else {
                        match step.run_as {
                            None => { Privilege::Usual }
                            Some(username) => { Privilege::AsUser(username) }
                        }
                    }
                }
            };

            match step.moduleblock.unwrap().dry_run_moduleblock(hosthandler, privilege, step.allowed_to_fail.unwrap_or(false)) {
                Ok((moduleblockchange, allowed_to_fail)) => {
                    mbchangeslist.push(moduleblockchange);
                    allowed_failures.push(allowed_to_fail);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        return Ok(TaskChange::from(mbchangeslist, allowed_failures));
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
    pub fn dry_run_tasklist(&self, _correlationid: String, hosthandler: &mut HostHandler) -> Result<ChangeList, Error> {
        let mut list: Vec<TaskChange> = Vec::new();

        for taskcontent in self.tasks.clone().iter() {
            match taskcontent.dry_run_task(hosthandler) {
                Ok(taskchange) => {
                    list.push(taskchange);
                }
                Err(e) => {
                    return Err(e)
                }
            }
        }
        return Ok(ChangeList::from(Some(list), hosthandler.clone()));
    }
}

// Any value that is present is considered Some value, including null. This way, we can use
// argument-less modules like Ping by writing "ping:" and Serde won't confuse it with a missing field.
fn deserialize_argumentlessmodule<'a, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'a>,
          D: Deserializer<'a>
{
    Deserialize::deserialize(deserializer).map(Some)
}