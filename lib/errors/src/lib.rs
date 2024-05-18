#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    FailureToParseContent(String),         // Used in TaskParser lib
    FailureToRunCommand(String),
    FailedInitialization(String),       // Used in Connection lib
    FailedTcpBinding(String),   // Used in Connection
    FailedTaskDryRun(String),
    MissingInitialization,      // Used for CorrelationId, RemoteHostHandler
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}

// Exit codes
// TODO : define global ranges per category of error, random values for now
pub const FAILURE_TO_OPEN_FILE: i32 = 15;
pub const FAILURE_TO_PARSE_FILE: i32 = 30;
