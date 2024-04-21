// Command module : <short description>

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::{ApiCallResult, ApiCallStatus};
use crate::modules::ModuleApiCall;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CommandBlockExpectedState {
    content: Option<String>,
}

impl CommandBlockExpectedState {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler, privilege: Privilege) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut changes: Vec<ModuleApiCall> = Vec::new();

        match &self.content {
            None => {
                changes.push(
                    ModuleApiCall::None(
                        String::from("No command to run")
                    )
                );
            }
            Some(cmdcontent) => {
                changes.push(ModuleApiCall::Command(
                    CommandApiCall {
                        cmd: cmdcontent.to_string(),
                        privilege
                    }
                ));
            }
        }
        return ModuleBlockChange::changes(changes);
    }

}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CommandApiCall {
    cmd: String,
    privilege: Privilege
}

impl CommandApiCall {

    pub fn display(&self) -> String {
        return format!("Run command : {}", self.cmd);
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let cmd_result = hosthandler.run_cmd(self.cmd.as_str(), self.privilege.clone()).unwrap();
        
        if cmd_result.exitcode == 0 {
            return ApiCallResult::from(
                Some(cmd_result.exitcode),
                Some(cmd_result.stdout),
                ApiCallStatus::ChangeSuccessful(
                    String::from("Command successfull")
                )
            );
        } else {
            return ApiCallResult::from(
                Some(cmd_result.exitcode),
                Some(cmd_result.stdout),
                ApiCallStatus::Failure(
                    String::from("Command failed")
                )
            );
        }
    }
}
