// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::workflow::result::{ModuleBlockResult, ModuleBlockStatus};
use crate::modules::ModuleBlockAction;
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
            // TODO : handle this case with an error
            return ModuleBlockChange::none();
        }

        let mut changes: Vec<ModuleBlockAction> = Vec::new();

        match &self.state {
            None => {}
            Some(state) => {
                match state.as_str() {
                    "present" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                
                        // Check is package is already installed or needs to be
                        if ! is_package_installed(hosthandler, self.package.clone().unwrap()) {
                            // Package is absent and needs to be installed
                            changes.push(
                                ModuleBlockAction::Apt(
                                    AptBlockAction::from("install", Some(self.package.clone().unwrap()))
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
                                ModuleBlockAction::Apt(
                                    AptBlockAction::from("remove", Some(self.package.clone().unwrap()))
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
                        ModuleBlockAction::Apt(
                            AptBlockAction::from("upgrade", None)
                        )
                    );
                }
            }
        }

        return ModuleBlockChange::from(Some(changes));
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AptBlockAction {
    action: String,
    package: Option<String>,
}

impl AptBlockAction {

    pub fn from(action: &str, package: Option<String>) -> AptBlockAction {
        AptBlockAction {
            action: action.to_string(),
            package
        }
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ModuleBlockResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut result = ModuleBlockResult::new();

        match self.action.as_str() {
            "install" => {
                let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get install -y {}", self.package.clone().unwrap());
                let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                
                if cmd_result.exitcode == 0 {
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ModuleBlockStatus::ChangeSuccessful(
                            format!("{} install successful", self.package.clone().unwrap())
                        ));
                } else {
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ModuleBlockStatus::ChangeFailed(
                            format!("{} install failed", self.package.clone().unwrap())
                        ));
                }
            }
            "remove" => {
                let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get autoremove -y {}", self.package.clone().unwrap());
                let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                
                if cmd_result.exitcode == 0 {
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ModuleBlockStatus::ChangeSuccessful(
                            format!("{} removal successful", self.package.clone().unwrap())
                        ));
                } else {
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ModuleBlockStatus::ChangeFailed(
                            format!("{} removal failed", self.package.clone().unwrap())
                        ));
                }
            }
            "upgrade" => {
                let cmd = "DEBIAN_FRONTEND=noninteractive apt-get update && apt-get upgrade -y";
                let cmd_result = hosthandler.run_cmd(cmd).unwrap();
                
                if cmd_result.exitcode == 0 {
                result = ModuleBlockResult::from(
                    Some(cmd_result.exitcode),
                    Some(cmd_result.stdout),
                    ModuleBlockStatus::ChangeSuccessful(
                        String::from("upgrade successful")
                    ));
                } else {
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        ModuleBlockStatus::ChangeFailed(
                            String::from("upgrade failed")
                        ));
                }
            }
            _ => {}
        }

        return result;
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