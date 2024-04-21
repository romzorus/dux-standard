use crate::prelude::ModuleApiCall;
// This part is used to generate an Assignment based on a TaskList and a HostList.
use crate::workflow::change::{ChangeList, ModuleBlockChange};
use crate::workflow::task::TaskList;
use crate::workflow::result::{ApiCallStatus, TaskListResult};
use connection::prelude::*;
use errors::Error;

#[derive(Clone)]
pub struct Assignment {
    pub correlationid: String,
    pub runningmode: RunningMode,
    pub host: String, // Will disappear soon, fully replaced by hosthandler
    pub hosthandler: HostHandler,
    pub tasklist: TaskList,
    pub changelist: ChangeList,
    pub tasklistresult: TaskListResult,
    pub finalstatus: AssignmentFinalStatus
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
            tasklistresult: TaskListResult::new(),
            finalstatus: AssignmentFinalStatus::Unset
        }
    }

    pub fn from(
        correlationid: String,
        runningmode: RunningMode,
        host: String,
        hosthandler: HostHandler,
        tasklist: TaskList,
        changelist: ChangeList,
        tasklistresult: TaskListResult,
        finalstatus: AssignmentFinalStatus
        ) -> Assignment {

            Assignment {
                correlationid,
                runningmode,
                host,
                hosthandler,
                tasklist,
                changelist,
                tasklistresult,
                finalstatus
            }
    }

    pub fn dry_run(&mut self) -> Result<(), Error> {

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

                match self.hosthandler.init() {
                    Ok(_) => {}
                    Err(e) => {
                        self.changelist.taskchanges = None;
                        self.finalstatus = AssignmentFinalStatus::FailedDryRun(format!("{:?}", e));
                        return Err(Error::FailedInitialization(format!("{:?}", e)));
                    }
                }

            }
        }
        
        let changelist = self.tasklist.dry_run_tasklist(self.correlationid.clone(), &mut self.hosthandler);
        match &changelist.taskchanges {
            Some(taskchangelist) => {
                let mut finalstatus = AssignmentFinalStatus::AlreadyMatched;
                for taskchange in taskchangelist {
                    for step in taskchange.stepchanges.clone() {
                        match step {
                            ModuleBlockChange::AlreadyMatched(_) => {}
                            ModuleBlockChange::FailedToEvaluate(e) => {
                                finalstatus = AssignmentFinalStatus::FailedDryRun(e);
                                break;
                            }
                            ModuleBlockChange::ModuleApiCalls(apicalllist) => {
                                for apicall in apicalllist {
                                    match apicall {
                                        ModuleApiCall::None(_) => {}
                                        _ => {
                                            finalstatus = AssignmentFinalStatus::Unset;
                                            break;
                                        }
                                    }
                                }

                            }
                        }
                    }
                }
                self.finalstatus = finalstatus;
            }
            None => {}
        }
        self.changelist = changelist;

        return Ok(());
    }
    
    // TODO : allow direct run with this method
    pub fn apply(&mut self) {
        assert_eq!(self.runningmode, RunningMode::Apply);
        assert_eq!(self.finalstatus, AssignmentFinalStatus::Unset);

        let tasklistresult = self.changelist.apply_changelist(&mut self.hosthandler);
        // "Save" the results
        self.tasklistresult = tasklistresult.clone();


        // Decide on the final status of the Assignment based on all the results
        // -> Considered successfull unless it failed at some point
        let mut finalstatus = AssignmentFinalStatus::Changed;
        for taskresult in tasklistresult.taskresults.iter() {
            for stepresult in taskresult.stepresults.as_ref().unwrap().iter() {
                for apicallresult in stepresult.apicallresults.iter() {
                    if let ApiCallStatus::ChangeFailed(_) = apicallresult.status {
                        finalstatus = AssignmentFinalStatus::FailedChange;
                    }
                }
            }
        }
        self.finalstatus = finalstatus;
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum RunningMode {
    DryRun, // Only check what needs to be done to match the expected situation
    Apply   // Actually apply the changes required to match the expected situation
}

#[derive(PartialEq, Debug, Clone)]
pub enum AssignmentFinalStatus {
    Unset,
    AlreadyMatched,
    FailedDryRun(String),
    Changed,
    FailedChange
}