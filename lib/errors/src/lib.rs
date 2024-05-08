#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    FailureToParseFile(String),         // Used in TaskParser lib
    FailureToRunCommand(String),
    FailedInitialization(String),       // Used in Connection lib
    FailedTcpBinding(String),   // Used in Connection
    FailedTaskDryRun(String),
    MissingInitialization,      // Used for CorrelationId, RemoteHostHandler
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}
