#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    FailureToParseFile,         // Used in TaskParser lib
    FailedInitialization(String),       // Used in Connection lib
    FailedTcpBinding(String),   // Used in Connection
    MissingInitialization,      // Used for CorrelationId, RemoteHostHandler
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}
