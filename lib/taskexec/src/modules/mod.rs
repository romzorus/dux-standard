pub mod blocks;
pub mod apt;
pub mod ping;
pub mod yumdnf;

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::modules::blocks::*;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ModuleBlockExpectedState {
    None, // Used for new() methods, initializations and errors
    Apt(AptBlockExpectedState),
    Dnf(YumDnfBlockExpectedState),
    Ping(PingBlockExpectedState),
    Yum(YumDnfBlockExpectedState)
}

impl ModuleBlockExpectedState {
    pub fn new() -> ModuleBlockExpectedState { ModuleBlockExpectedState::None }

    pub fn dry_run_moduleblock(&self, hosthandler: &mut HostHandler, privilege: Privilege) -> ModuleBlockChange {
        match &self {
            ModuleBlockExpectedState::None => { ModuleBlockChange::matched("none") }
            ModuleBlockExpectedState::Apt(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Dnf(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Ping(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Yum(block) => { block.dry_run_block(hosthandler, privilege) }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleApiCall {
    None(String),
    Apt(AptApiCall),
    Ping(PingApiCall),
    YumDnf(YumDnfApiCall)
}
