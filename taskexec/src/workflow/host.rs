// This part is used to generate a HostList based on the user input.

pub struct HostList {
    hostlist: Vec<Host>,
}

pub struct Host {
    pub address: String,
}

impl HostList {
    pub fn new() -> HostList {
        HostList {
            hostlist: Vec::<Host>::new(),
        }
    }
}
