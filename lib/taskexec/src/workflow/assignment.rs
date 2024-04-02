// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::ChangeList;
use crate::workflow::task::TaskList;
use crate::workflow::result::TaskListResult;
use connection::prelude::*;

pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String, // Will disappear soon, fully replaced by hosthandler
    pub hosthandler: HostHandler,
    pub tasklist: TaskList,
    pub changelist: ChangeList,
    pub tasklistresult: TaskListResult
}

impl Assignment {
    pub fn new(correlationid: String) -> Assignment {
        Assignment {
            correlationid,
            runningmode: RunningMode::DryRun, // DryRun is default running mode
            host: String::from(""),
            hosthandler: HostHandler::new(),
            tasklist: TaskList::new(),
            changelist: ChangeList::new(),
            tasklistresult: TaskListResult::new()
        }
    }

    pub fn from(
        correlationid: String,
        runningmode: RunningMode,
        host: String,
        hosthandler: HostHandler,
        tasklist: TaskList,
        changelist: ChangeList,
        tasklistresult: TaskListResult
        ) -> Assignment {

            Assignment {
                correlationid,
                runningmode,
                host,
                hosthandler,
                tasklist,
                changelist,
                tasklistresult
            }
    }

    pub fn dry_run(&mut self) {

        // TODO : turn all this connection initialization into an Assignment's method
        match &self.hosthandler.connectionmode {
            ConnectionMode::Unset => {} // TODO : return some error
            ConnectionMode::LocalHost => {} // Nothing to initialize if working on the localhost
            ConnectionMode::Ssh2 => {
                match self.hosthandler.ssh2.authmode.clone() {
                    Ssh2AuthMode::Unset => {} // TODO : return some error, missing auth mode
                    Ssh2AuthMode::UsernamePassword(_credentials) => {} // TODO : handle connection with username/password
                    Ssh2AuthMode::SshKeys((username, privatekeypath)) => {
                        self.hosthandler = HostHandler::from(ConnectionMode::Ssh2, self.hosthandler.hostaddress.clone());
                        self.hosthandler.ssh2auth(Ssh2AuthMode::SshKeys((username.clone(), privatekeypath.to_path_buf())));
                    }
                    Ssh2AuthMode::SshAgent(_agentname) => {} // TODO : handle connection with agent
                }

                self.hosthandler.init().expect("Failed HostHandler initialization");
            }
        }
        
        let changelist = self.tasklist.dry_run_tasklist(self.correlationid.clone(), &mut self.hosthandler);

        self.changelist = changelist;
    }
    
    // TODO : allow direct run with this method
    pub fn apply(&mut self) {
        assert_eq!(self.runningmode, RunningMode::Apply);
        
        let tasklistresult = self.changelist.apply_changelist(&mut self.hosthandler);

        self.tasklistresult = tasklistresult;
    }
}

#[derive(PartialEq, Debug)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}