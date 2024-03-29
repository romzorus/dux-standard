use std::path::PathBuf;
use ssh2::Session;
use std::net::TcpStream;
use errors::Error;

pub struct Ssh2HostHandler {
    pub hostaddress: String,
    pub sshsession: Session,
    pub authmode: Ssh2AuthMode,
}

impl Ssh2HostHandler {
    pub fn new() -> Ssh2HostHandler {
        Ssh2HostHandler {
            hostaddress: String::new(),
            sshsession: Session::new().unwrap(),
            authmode: Ssh2AuthMode::Unset,
        }
    }

    pub fn init(&mut self, hostaddress: String, authmode: Ssh2AuthMode) -> Result<(), Error> {
        match authmode {
            Ssh2AuthMode::Unset => { return Err(Error::MissingInitialization) }
            Ssh2AuthMode::UsernamePassword(credentials) => { return Ok(()); } // TODO
            Ssh2AuthMode::SshKeys((username, privatekey)) => {
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
            Ssh2AuthMode::SshAgent(agent) => { return Ok(()); } // TODO
        }
    }
}


#[derive(Debug, Clone)]
pub enum Ssh2AuthMode {
    Unset,
    UsernamePassword(Credentials),
    SshKeys((String, PathBuf)),   // (username, private key's path)
    SshAgent(String)    // Name of SSH agent
}

#[derive(Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String
}