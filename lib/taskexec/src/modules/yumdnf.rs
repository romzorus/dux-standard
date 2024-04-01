// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::{change::ModuleBlockChange, result::ModuleBlockResult};
use crate::modules::ModuleBlock;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct YumDnfBlock {
    state: Option<String>,
    package: Option<String>,
    upgrade: Option<bool>
}

impl YumDnfBlock {
    pub fn dry_run_block(&self, hosthandler: &mut HostHandler) -> ModuleBlockChange {
        assert!(hosthandler.ssh2.sshsession.authenticated());

        let mut tool = String::new();

        if is_dnf_working(hosthandler) {
            tool = String::from("dnf");
        } else if is_yum_working(hosthandler) {
            tool = String::from("yum");
        } else {
            println!("[DRY-RUN] : YUM and DNF absent or not working properly on {}", hosthandler.hostaddress);
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
                        if ! is_package_installed(hosthandler, tool, self.package.clone().unwrap()) {
                            // Package is absent and needs to be installed
                            changes.push(
                                    ModuleBlock::Dnf(YumDnfBlock{
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
                        if is_package_installed(hosthandler, tool, self.package.clone().unwrap()) {
                            // Package is present and needs to be removed
                            changes.push(
                                    ModuleBlock::Dnf(YumDnfBlock{
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
                            ModuleBlock::Dnf(YumDnfBlock{
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

        let mut tool = String::new();

        if is_dnf_working(hosthandler) {
            tool = String::from("dnf");
        } else if is_yum_working(hosthandler) {
            tool = String::from("yum");
        } else {
            println!("[APPLY] : YUM and DNF absent or not working properly on {}", hosthandler.hostaddress);
            return ModuleBlockResult::none();
        }

        let mut result = ModuleBlockResult::new();

        match &self.state {
            None => {}
            Some(state) => {
                match state.as_str() {
                    "install" => {                              
                        let cmd = format!("{tool} install -y {}", self.package.clone().unwrap());
                        let cmd_result = hosthandler.run_cmd(cmd.as_str()).unwrap();
                        
                        result = ModuleBlockResult::from(
                            Some(cmd_result.exitcode),
                            Some(cmd_result.stdout),
                            None)
        
                    }
                    "remove" => {
                        let cmd = format!("{tool} remove -y {}", self.package.clone().unwrap());
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
                    let cmd = "{tool} update --refresh";
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