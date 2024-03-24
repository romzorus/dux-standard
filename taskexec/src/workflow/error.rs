#[derive(Debug)]
pub enum Error {
    FailureToFindGroupContent,  // Used in HostParser lib by hostlist_get_specific_group()
    MissingInitialization,      // Used for CorrelationId
    GroupNotFound,              // Used in HostParser lib by hostlist_get_specific_group()
    MissingGroupsList,          // Used in HostParser lib by hostlist_get_specific_group()
    
}