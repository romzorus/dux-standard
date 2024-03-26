use serde::Deserialize;
use std::path::PathBuf;
use ssh2::Session;
use std::net::TcpStream;
use taskexec::workflow::error::Error;

pub struct RemoteHostHandler {
    hostaddress: String,
    sshsession: Session,
    connectionmode: ConnectionMode,
}

impl RemoteHostHandler {
    pub fn new() -> RemoteHostHandler {
        RemoteHostHandler {
            hostaddress: String::new(),
            sshsession: Session::new().unwrap(),
            connectionmode: ConnectionMode::Unset,
        }
    }

    pub fn init(&mut self, hostaddress: String, connectionmode: ConnectionMode) -> Result<(), Error> {
        match connectionmode {
            ConnectionMode::Unset => { return Err(Error::MissingInitialization) }
            ConnectionMode::UsernamePassword(credentials) => { return Ok(()); } // TODO
            ConnectionMode::SshKeys((username, privatekey)) => {
                let tcp = TcpStream::connect(format!("{}:22", hostaddress)).unwrap(); // TODO : add SSH custom port handling
                self.sshsession.set_tcp_stream(tcp);
                self.sshsession.handshake().unwrap();
                self.sshsession.userauth_pubkey_file(username.as_str(), None, &privatekey, None).unwrap(); // TODO : add pubkey and passphrase support

                if self.sshsession.authenticated() {
                    return Ok(());
                } else {
                    return Err(Error::FailedInitialization)
                }
            }
            ConnectionMode::SshAgent(agent) => { return Ok(()); } // TODO
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub enum ConnectionMode {
    Unset,
    UsernamePassword(Credentials),
    SshKeys((String, PathBuf)),   // (username, path to private key)
    SshAgent(String)    // Name of SSH agent
}

#[derive(Debug, Deserialize, Clone)]
pub struct Credentials {
    username: String,
    password: String
}