use errors::Error;
use crate::ssh2mode::{Ssh2AuthMode, Ssh2HostHandler};

pub mod prelude;
pub mod ssh2mode;

#[derive(Debug, Clone)]
pub enum ConnectionMode {
    Unset,
    LocalHost,
    Ssh2,
    // Ssh3
}

#[derive(Clone)]
pub struct HostHandler {
    pub connectionmode: ConnectionMode,
    pub hostaddress: String,
    pub ssh2: Ssh2HostHandler,
    // ssh3handler: Ssh3HostHandler
}

impl HostHandler {
    pub fn new() -> HostHandler {
        HostHandler {
            connectionmode: ConnectionMode::Unset,
            hostaddress: String::new(),
            ssh2: Ssh2HostHandler::new(),
            // ssh3handler: ....
        }
    }

    pub fn from(connectionmode: ConnectionMode, hostaddress: String) -> HostHandler {
        HostHandler {
            connectionmode,
            hostaddress: hostaddress.clone(),
            ssh2: Ssh2HostHandler::from(hostaddress, Ssh2AuthMode::Unset),
            // ssh3: ...
        }
    }
    
    pub fn ssh2auth(&mut self, authmode: Ssh2AuthMode) {
        self.ssh2.authmode = authmode;
    }
    
    pub fn init(&mut self) -> Result<(), Error> {
        match self.connectionmode {
            ConnectionMode::Unset => { return Err(Error::MissingInitialization); }
            ConnectionMode::LocalHost => { return Ok(()); } // Nothing to initialize when working on localhost
            ConnectionMode::Ssh2 => { self.ssh2.init() }
            // ConnectionMode::Ssh3 => { self.ssh2.unwrap().init() }
        }
    }
    
    pub fn run_cmd(&mut self, cmd: &str) -> Result<String, Error> {
        match self.connectionmode {
            ConnectionMode::Unset => { return Err(Error::MissingInitialization); }
            ConnectionMode::LocalHost => { return Ok(String::from("PLACEHOLDER")); } // Nothing to initialize when working on localhost
            ConnectionMode::Ssh2 => { self.ssh2.run_cmd(cmd) }
            // ConnectionMode::Ssh3 => { self.ssh3.unwrap().run_cmd() }
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

