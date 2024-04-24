// **BEACON_1**
pub mod lineinfile;
pub mod command;
pub mod blocks;
pub mod apt;
pub mod ping;
pub mod yumdnf;

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::modules::blocks::*;
use connection::prelude::*;
use errors::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ModuleBlockExpectedState {
    None, // Used for new() methods, initializations and errors
// **BEACON_2**
    LineInFile(LineInFileBlockExpectedState),
    Command(CommandBlockExpectedState),
    Apt(AptBlockExpectedState),
    Dnf(YumDnfBlockExpectedState),
    Ping(PingBlockExpectedState),
    Yum(YumDnfBlockExpectedState)
}

impl ModuleBlockExpectedState {
    pub fn new() -> ModuleBlockExpectedState { ModuleBlockExpectedState::None }

    pub fn dry_run_moduleblock(&self, hosthandler: &mut HostHandler, privilege: Privilege, allowed_to_fail: bool) -> Result<(ModuleBlockChange, bool), Error> {
        
        let mbchange: ModuleBlockChange = match &self {
            ModuleBlockExpectedState::None => { ModuleBlockChange::matched("none") }
// **BEACON_3**
            ModuleBlockExpectedState::LineInFile(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Command(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Apt(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Dnf(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Ping(block) => { block.dry_run_block(hosthandler, privilege) }
            ModuleBlockExpectedState::Yum(block) => { block.dry_run_block(hosthandler, privilege) }
        };

        match mbchange {
            ModuleBlockChange::FailedToEvaluate(message) => {
                if allowed_to_fail {
                    return Ok((ModuleBlockChange::AllowedFailure(message), allowed_to_fail));
                } else {
                    return Err(Error::FailedTaskDryRun(message));
                }
            }
            _ => { return Ok((mbchange, allowed_to_fail)); }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleApiCall {
    None(String),
// **BEACON_4**
    LineInFile(LineInFileApiCall),
    Command(CommandApiCall),
    Apt(AptApiCall),
    Ping(PingApiCall),
    YumDnf(YumDnfApiCall)
}
