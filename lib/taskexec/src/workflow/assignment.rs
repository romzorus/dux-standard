// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::task::TaskList;
use crate::workflow::result::TaskListResult;
use connection::prelude::*;
use connection::ssh2mode::{Ssh2AuthMode, Ssh2HostHandler};
use std::path::{Path, PathBuf};

pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String, // Will disappear soon, fully replaced by hosthandler
    pub tasklist: TaskList,
    pub hosthandler: HostHandler
}

impl Assignment {
    pub fn new(correlationid: String) -> Assignment {
        Assignment {
            correlationid,
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: String::from(""),
            tasklist: TaskList::new(),
            hosthandler: HostHandler::new()
        }
    }

    pub fn from(
        correlationid: String,
        runningmode: RunningMode,
        host: String,
        tasklist: TaskList,
        hosthandler: HostHandler) -> Assignment {
        Assignment {
            correlationid,
            runningmode,
            host,
            tasklist,
            hosthandler
        }
    }

    pub fn dry_run(&mut self) -> ChangeList {

        // Initialization of the connection based on CLI args, var files...etc
        // For now, a local struct is defined to simulate those parameters
        struct HostParameters {
            connectionmode: ConnectionMode,
            ssh2authmode: Ssh2AuthMode,
        }

        let hostparams = HostParameters {
            connectionmode: ConnectionMode::Ssh2,
            ssh2authmode: Ssh2AuthMode::SshKeys((
                String::from("root"),
                PathBuf::from("/home/romzor/Developpement/dux/testing/docker/controller_key")
            ))
        };

        match hostparams.connectionmode {
            ConnectionMode::Ssh2 => {
                // The ssh2auth field needs to be set before running init()
                match hostparams.ssh2authmode {
                    Ssh2AuthMode::Unset => {} // TODO : return some error, missing auth mode
                    Ssh2AuthMode::UsernamePassword(_credentials) => {}
                    Ssh2AuthMode::SshKeys((username, privatekeypath)) => {
                        self.hosthandler = HostHandler::from(ConnectionMode::Ssh2, self.hosthandler.hostaddress.clone());
                        self.hosthandler.ssh2auth(Ssh2AuthMode::SshKeys((username, privatekeypath.to_path_buf())));
                    }
                    Ssh2AuthMode::SshAgent(_agentname) => {}
                }

                // Now running init()
                self.hosthandler.init().expect("Failed HostHandler initialization");
            }
            ConnectionMode::LocalHost => {} // Nothing to initialize if working on the localhost
            _ => {}
        }
        
        self.tasklist.dry_run_tasklist(self.correlationid.clone(), &mut self.hosthandler)
        
    }

    pub fn apply(&self) -> TaskListResult {

        TaskListResult::new(self.correlationid.clone())

    }
}

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}
