#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    FailedInitialization,
    MissingInitialization,      // Used for CorrelationId, RemoteHostHandler
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}