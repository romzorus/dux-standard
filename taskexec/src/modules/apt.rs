// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::{change::ModuleBlockChange, result::{ModuleBlockResult, TaskResult}};
use crate::modules::ModuleBlock;
use connection::prelude::*;
use std::path::Path;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AptBlock {
    action: String,
    package: Option<String>,
}

impl AptBlock {
    pub fn dry_run_block(&self, host: String) -> ModuleBlockChange {
        match self.action.as_str() {
            "install" => {
                ModuleBlockChange {
                    module: Some(
                        ModuleBlock::Apt(AptBlock{
                            action: "install".to_string(),
                            package: Some(self.package.clone().unwrap())
                        })
                    )
                }
            }
            "remove" => { ModuleBlockChange::new_none() }
            "update" => { ModuleBlockChange::new_none() }
            _ => { ModuleBlockChange::new_none() }
        }
    }

    pub fn apply_moduleblock_change(&self, host: String) -> ModuleBlockResult {
        match self.action.as_str() {
            "install" => {
                println!("**** Install package {}", self.package.clone().unwrap());
                let privatekey = Path::new("/home/romzor/Developpement/dux/testing/docker/controller_key");
                let mut remotehosthandler = RemoteHostHandler::new();
        
                let _ = remotehosthandler.init(
                    host,
                    ConnectionMode::SshKeys((
                            "root".to_string(),
                            privatekey.to_path_buf()))
                    );
        
                assert!(remotehosthandler.sshsession.authenticated());
        
                let mut channel = remotehosthandler.sshsession.channel_session().unwrap();
                channel.exec("cat /etc/os-release | grep ^NAME").unwrap();
                let mut s = String::new();
                channel.read_to_string(&mut s).unwrap();
                let _ = channel.wait_close();
                ModuleBlockResult::from(
                    Some(0),
                    Some(s),
                    None)

            }
            "remove" => {
                println!("**** Remove package {}", self.package.clone().unwrap());
                ModuleBlockResult::new_none()
            }
            _ => { ModuleBlockResult::new_none() }
        }
    }
}