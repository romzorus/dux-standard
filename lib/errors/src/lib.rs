#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    FailedInitialization,       // Used in Connection lib
    MissingInitialization,      // Used for CorrelationId, RemoteHostHandler
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}