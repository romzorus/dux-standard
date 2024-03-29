use errors::Error;

pub mod prelude;
pub mod ssh2mode;

#[derive(Debug, Clone)]
pub enum ConnectionMode {
    Unset,
    LocalHost,
    Ssh2,
    Ssh3
}

#[derive(Debug, Clone)]
pub struct HostHandler {
    connectionmode: ConnectionMode,
    hostaddress: String,
}

impl HostHandler {
    pub fn new() -> HostHandler {
        HostHandler {
            connectionmode: ConnectionMode::Unset,
            hostaddress: String::new()
        }
    }

    pub fn from(connectionmode: ConnectionMode, hostaddress: &String) -> HostHandler {
        HostHandler {
            connectionmode,
            hostaddress: hostaddress.clone()
        }
    }
}

// This trait is defined to normalize methods on HostHandlers
// based on several connection modes : localhost, SSHv2, (SSHv3 ?)
trait HostHandling {
    fn new() -> Self; // Create new instance of the handler
    fn init() -> Result<(), Error> where Self:Sized; // Initialize the handler (when needed)
    fn run_cmd(); // Run a shell command on the host
    fn run_cmd_with_bkp(); // Run a shell command on the host, with an alternative command in case the first one fails
    fn put_file(); // Upload a file to the host
    fn get_file(); // Download a file from the host
}