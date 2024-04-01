// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::task::TaskList;
use crate::workflow::result::TaskListResult;
use connection::prelude::*;

pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String, // Will disappear soon, fully replaced by hosthandler
    pub tasklist: TaskList,
    pub hosthandler: HostHandler,
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

        // TODO : turn all this connection initialization into anAssignment's method
        match &self.hosthandler.connectionmode {
            ConnectionMode::Unset => {} // TODO : return some error
            ConnectionMode::LocalHost => {} // Nothing to initialize if working on the localhost
            ConnectionMode::Ssh2 => {
                match self.hosthandler.ssh2.authmode.clone() {
                    Ssh2AuthMode::Unset => {} // TODO : return some error, missing auth mode
                    Ssh2AuthMode::UsernamePassword(_credentials) => {}
                    Ssh2AuthMode::SshKeys((username, privatekeypath)) => {
                        self.hosthandler = HostHandler::from(ConnectionMode::Ssh2, self.hosthandler.hostaddress.clone());
                        self.hosthandler.ssh2auth(Ssh2AuthMode::SshKeys((username.clone(), privatekeypath.to_path_buf())));
                    }
                    Ssh2AuthMode::SshAgent(_agentname) => {}
                }

                self.hosthandler.init().expect("Failed HostHandler initialization");
            }
        }
        
        self.tasklist.dry_run_tasklist(self.correlationid.clone(), &mut self.hosthandler)
        
    }
    
    // TODO : allow direct run with this method
    pub fn apply(&self) -> TaskListResult {
        assert_eq!(self.runningmode, RunningMode::Apply);
        TaskListResult::new(self.correlationid.clone()) // PLACEHOLDER

    }
}

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}