// YUM / DNF Module : handle packages in Fedora-like distributions

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::{ApiCallResult, ApiCallStatus};
use crate::modules::ModuleApiCall;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YumDnfBlockExpectedState {
    state: Option<String>,
    package: Option<String>,
    upgrade: Option<bool>
}

impl YumDnfBlockExpectedState {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut tool = String::new();

        if is_dnf_working(hosthandler) {
            tool = String::from("dnf");
        } else if is_yum_working(hosthandler) {
            tool = String::from("yum");
        } else {
            return ModuleBlockChange::failed_to_evaluate("Neither YUM nor DNF work on this host");
        }

        let mut changes: Vec<ModuleApiCall> = Vec::new();

        match &self.state {
            None => {}
            Some(state) => {
                match state.as_str() {
                    "present" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                
                        // Check is package is already installed or needs to be
                        if ! is_package_installed(hosthandler, tool, self.package.clone().unwrap()) {
                            // Package is absent and needs to be installed
                            changes.push(
                                ModuleApiCall::YumDnf(
                                    YumDnfApiCall::from("install", Some(self.package.clone().unwrap()))
                                )
                            );
                        }
                    }
                    "absent" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                
                        // Check is package is already absent or needs to be removed
                        if is_package_installed(hosthandler, tool, self.package.clone().unwrap()) {
                            // Package is present and needs to be removed
                            changes.push(
                                ModuleApiCall::YumDnf(
                                    YumDnfApiCall::from("remove", Some(self.package.clone().unwrap()))
                                )
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        match self.upgrade {
            None => {}
            Some(value) => {
                if value {
                    changes.push(
                        ModuleApiCall::YumDnf(
                            YumDnfApiCall::from("upgrade", None)
                        )
                    );
                }
            }
        }

        if changes.is_empty() {
            return ModuleBlockChange::matched("Expected state already matched");
        } else {
            return ModuleBlockChange::changes(changes);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YumDnfApiCall {
    action: String,
    package: Option<String>,
}

impl YumDnfApiCall {

    pub fn from(action: &str, package: Option<String>) -> YumDnfApiCall {
        YumDnfApiCall {
            action: action.to_string(),
            package
        }
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut tool = String::new();

        if is_dnf_working(hosthandler) {
            tool = String::from("dnf");
        } else if is_yum_working(hosthandler) {
            tool = String::from("yum");
        } else {
            // TODO : handle this case with an error
            return ApiCallResult::none();
        }

        match self.action.as_str() {
            "install" => {
                let cmd = format!("{tool} install -y {}", self.package.clone().unwrap());
                let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                
                if cmd_result.exitcode == 0 {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeSuccessful(
                            format!("{} install successful", self.package.clone().unwrap())
                        )
                    );
                } else {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeFailed(
                            format!("{} install failed", self.package.clone().unwrap())
                        )
                    );
                }
            }
            "remove" => {
                let cmd = format!("{tool} remove -y {}", self.package.clone().unwrap());
                let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                
                if cmd_result.exitcode == 0 {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeSuccessful(
                            format!("{} removal successful", self.package.clone().unwrap())
                        )
                    );
                } else {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeFailed(
                            format!("{} removal failed", self.package.clone().unwrap())
                        )
                    );
                }
            }
            "upgrade" => {
                let cmd = format!("{tool} update --refresh");
                let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                
                if cmd_result.exitcode == 0 {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeSuccessful(String::from("Yum/DNF upgrade successful"))
                    );
                    } else {
                        println!("------{}", cmd_result.stdout);
                        return ApiCallResult::from(
                            Some(cmd_result.exitcode),
                            Some(cmd_result.stdout),
                            ApiCallStatus::ChangeFailed(String::from("Yum/DNF upgrade failed"))
                        );
                    }
            }
            _ => { return ApiCallResult::none(); }
        }
    }
}

fn is_dnf_working(hosthandler: &mut HostHandler) -> bool {

    let cmd = "dnf";
    let cmd_result = hosthandler.run_cmd(cmd).unwrap();

    if cmd_result.exitcode == 0 {
        return true;
    } else {
        return false;
    }
}

fn is_yum_working(hosthandler: &mut HostHandler) -> bool {

    let cmd = "yum";
    let cmd_result = hosthandler.run_cmd(cmd).unwrap();

    if cmd_result.exitcode == 0 {
        return true;
    } else {
        return false;
    }
}

fn is_package_installed(hosthandler: &mut HostHandler, tool: String, package: String) -> bool {
    let test = hosthandler.run_cmd(
        format!("{tool} list installed {}", package).as_str()
    ).unwrap();

    if test.exitcode == 0 {
        return true;
    } else {
        return false;
    }
}