// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::{ApiCallResult, ApiCallStatus};
use crate::modules::ModuleApiCall;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AptBlockExpectedState {
    state: Option<String>,
    package: Option<String>,
    upgrade: Option<bool>
}

impl AptBlockExpectedState {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        if ! is_apt_working(hosthandler) {
            return ModuleBlockChange::failed_to_evaluate("APT not working on this host");
        }

        let mut changes: Vec<ModuleApiCall> = Vec::new();

        match &self.state {
            None => {}
            Some(state) => {
                match state.as_str() {
                    "present" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                        
                        // Check is package is already installed or needs to be
                        if is_package_installed(hosthandler, self.package.clone().unwrap()) {
                            changes.push( ModuleApiCall::None(
                                format!("{} already present", self.package.clone().unwrap())
                                )
                            );
                        } else {
                            // Package is absent and needs to be installed
                            changes.push(
                                ModuleApiCall::Apt(
                                    AptApiCall::from("install", Some(self.package.clone().unwrap()))
                                )
                            );
                        }
                    }
                    "absent" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                
                        // Check is package is already absent or needs to be removed
                        if is_package_installed(hosthandler, self.package.clone().unwrap()) {
                            // Package is present and needs to be removed
                            changes.push(
                                ModuleApiCall::Apt(
                                    AptApiCall::from("remove", Some(self.package.clone().unwrap()))
                                )
                            );
                        } else {
                            changes.push( ModuleApiCall::None(
                                format!("{} already absent", self.package.clone().unwrap())
                                )
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(value) = self.upgrade {
            if value {
                changes.push(
                    ModuleApiCall::Apt(
                        AptApiCall::from("upgrade", None)
                    )
                );
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
pub struct AptApiCall {
    action: String,
    package: Option<String>,
}

impl AptApiCall {

    pub fn display(&self) -> String {
        match self.action.as_str() {
            "install" => {
                return format!("Install - {}", self.package.clone().unwrap());
            }
            "remove" => {
                return format!("Remove - {}", self.package.clone().unwrap());
            }
            "upgrade" => {
                return String::from("Upgrade");
            }
            _ => { return String::from("Wrong AptApiCall action"); }
        }
    }

    pub fn from(action: &str, package: Option<String>) -> AptApiCall {
        AptApiCall {
            action: action.to_string(),
            package
        }
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ApiCallResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        match self.action.as_str() {
            "install" => {
                let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get install -y {}", self.package.clone().unwrap());
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
                let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get autoremove -y {}", self.package.clone().unwrap());
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
                let cmd = "DEBIAN_FRONTEND=noninteractive apt-get update && apt-get upgrade -y";
                let cmd_result = hosthandler.run_cmd(cmd).unwrap();
                
                if cmd_result.exitcode == 0 {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeSuccessful(String::from("APT upgrade successful"))
                    );
                } else {
                    return ApiCallResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ApiCallStatus::ChangeFailed(String::from("APT upgrade failed"))
                    );
                }
            }
            _ => { return ApiCallResult::none(); }
        }
    }
}

fn is_apt_working(hosthandler: &mut HostHandler) -> bool {

    let cmd = "apt-get check";
    let cmd_result = hosthandler.run_cmd(cmd).unwrap();

    if cmd_result.exitcode != 0 {
        return false;
    } else {
        return true;
    }
}

fn is_package_installed(hosthandler: &mut HostHandler, package: String) -> bool {
    let test = hosthandler.run_cmd(
        format!("apt-cache policy {}", package).as_str()
    ).unwrap();

    match test.stdout.find("Installed: (none)") {
        Some(_) => { return false; }
        None => { return true; }
    }
}
