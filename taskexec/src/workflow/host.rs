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

    pub fn from(address: &str) -> Host {
        Host {
            address: address.to_string()
        }
    }
}

#[derive(Debug)]
pub struct HostList {
    list: Vec<Host>,
}

impl HostList {
    pub fn new() -> HostList {
        HostList {
            list: Vec::<Host>::new(),
        }
    }

    pub fn from(list: Vec<Host>) -> HostList {
        HostList {
            list
        }
    }
}
