// This part is used to generate a HostList based on the user input.

#[derive(Debug)]
pub struct Host {
    pub address: String,
}

impl Host {
    pub fn new() -> Host {
        Host {
            address: String::from("0.0.0.0")
        }
    }
}

#[derive(Debug)]
pub struct HostList {
    hostlist: Vec<Host>,
}

impl HostList {
    pub fn new() -> HostList {
        HostList {
            hostlist: Vec::<Host>::new(),
        }
    }
}
