// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::{ApiCallResult, ApiCallStatus};
use crate::modules::ModuleApiCall;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PingBlockExpectedState {}

impl PingBlockExpectedState {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler, privilege: Privilege) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        return ModuleBlockChange::changes(
            vec![ModuleApiCall::Ping(PingApiCall{privilege})]
        );

    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PingApiCall {
    privilege: Privilege
}

impl PingApiCall {

    pub fn display(&self) -> String {
        return format!("Check SSH connectivity with remote host");
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let cmd = String::from("DEBIAN_FRONTEND=noninteractive id");
        let cmd_result = hosthandler.run_cmd(cmd.as_str(), self.privilege.clone()).unwrap();
        
        if cmd_result.exitcode == 0 {
            return ApiCallResult::from(
                Some(cmd_result.exitcode),
                Some(cmd_result.stdout),
                ApiCallStatus::ChangeSuccessful(
                    format!("Host reachable through SSH")
                )
            );
        } else {
            return ApiCallResult::from(
                Some(cmd_result.exitcode),
                Some(cmd_result.stdout),
                ApiCallStatus::ChangeFailed(
                    format!("Host unreachable through SSH")
                )
            );
        }
        
    }
}
