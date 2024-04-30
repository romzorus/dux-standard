// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::ApiCallResult;
use crate::modules::{DryRun, Apply};
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PingBlockExpectedState {}

impl DryRun for PingBlockExpectedState {
    fn dry_run_block(&self, hosthandler: &mut HostHandler, privilege: Privilege) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let cmd = String::from("DEBIAN_FRONTEND=noninteractive id");
        let cmd_result = hosthandler.run_cmd(cmd.as_str(), privilege).unwrap();

        if cmd_result.exitcode == 0 {
            return ModuleBlockChange::AlreadyMatched("Host reachable".to_string());
        } else {
            return ModuleBlockChange::FailedToEvaluate("Host unreachable".to_string());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct PingApiCall {
    privilege: Privilege
}

impl Apply for PingApiCall {

    fn display(&self) -> String {
        return format!("Check SSH connectivity with remote host");
    }

    fn apply_moduleblock_change(&self, _hosthandler: &mut HostHandler) -> ApiCallResult {
        return ApiCallResult::none();
    }
}
