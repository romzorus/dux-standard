// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::{change::ModuleBlockChange, result::ModuleBlockResult};
use crate::modules::ModuleBlock;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AptBlock {
    state: Option<String>,
    package: Option<String>,
    upgrade: Option<bool>
}

impl AptBlock {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        if ! is_apt_working(hosthandler) {
            println!("[DRY-RUN] : APT absent or not working properly on {}", hosthandler.hostaddress);
            return ModuleBlockChange::none();
        }

        let mut changes: Vec<ModuleBlock> = Vec::new();

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
                                    ModuleBlock::Apt(AptBlock{
                                        state: Some("install".to_string()),
                                        package: Some(self.package.clone().unwrap()),
                                        upgrade: None
                                    })
                                );
                        }
                    }
                    "absent" => {
                        assert!(hosthandler.ssh2.sshsession.authenticated());
                
                        // Check is package is already absent or needs to be removed
                        if is_package_installed(hosthandler, self.package.clone().unwrap()) {
                            // Package is present and needs to be removed
                            changes.push(
                                    ModuleBlock::Apt(AptBlock{
                                        state: Some("remove".to_string()),
                                        package: Some(self.package.clone().unwrap()),
                                        upgrade: None
                                    })
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
                            ModuleBlock::Apt(AptBlock{
                                state: None,
                                package: None,
                                upgrade: Some(true)
                                })
                            );
                }
            }
        }

        return ModuleBlockChange::from(Some(changes));
    }

    pub fn apply_moduleblock_change(&self, hosthandler: &mut HostHandler) -> ModuleBlockResult {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut result = ModuleBlockResult::new();

        match &self.state {
            None => {}
            Some(state) => {
                match state.as_str() {
                    "install" => {                              
                        let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get install -y {}", self.package.clone().unwrap());
                        let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                        
                        result = ModuleBlockResult::from(
                            Some(cmd_result.exitcode),
                            Some(cmd_result.stdout),
                            None)
        
                    }
                    "remove" => {
                        let cmd = format!("DEBIAN_FRONTEND=noninteractive apt-get autoremove -y {}", self.package.clone().unwrap());
                        let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                        
                        result = ModuleBlockResult::from(
                            Some(cmd_result.exitcode),
                            Some(cmd_result.stdout),
                            None)
                    }
                    _ => {}
                }
            }
        }

        match self.upgrade {
            None => {}
            Some(value) => {
                if value {
                    let cmd = "DEBIAN_FRONTEND=noninteractive apt-get update && apt-get upgrade -y";
                    let cmd_result = hosthandler.run_cmd(cmd).unwrap();
                    
                    result = ModuleBlockResult::from(
                        Some(cmd_result.exitcode),
                        Some(cmd_result.stdout),
                        None);
                }
            }
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