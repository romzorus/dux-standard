use std::collections::HashMap;
use serde::Deserialize;
use errors::Error;

// TODO : define Host ? any interest ?
// pub struct Host {
//     ...
// }
#[derive(Debug, Deserialize, Clone)]
pub struct Host {
    address: String,
    vars: Option<HashMap<String, String>>
}

impl Host {
    pub fn from_string(address: String) -> Host {
        Host {
            address,
            vars: None
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostListVarsUnparsed {
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<String>>,
    pub groups: Option<Vec<Group>>
}

impl HostListVarsUnparsed {
    pub fn parse(&self) -> HostList {

        match &self.hosts {
            Some(hosts_list) => {

                let mut parsed_hosts: Vec<Host> = Vec::new();

                for host_string in hosts_list {
                    let mut line = host_string.split(['[', ']']);
                    let hostname = line.next().unwrap().trim();

                    match line.next() {
                        Some(vars_content) => {
                            let mut vars_list: HashMap<String, String> = HashMap::new();
                            for vardef in vars_content.split(',') {
                                let mut vardef_parsed = vardef.split('=');
                                let key = vardef_parsed.next().unwrap().trim();
                                let value = vardef_parsed.next().unwrap().trim();
                                vars_list.insert(key.to_string(), value.to_string());
                            }
                            parsed_hosts.push(Host {
                                address: hostname.to_string(),
                                vars: Some(vars_list)
                            })
                        }
                        None => {
                            parsed_hosts.push(Host {
                                address: hostname.to_string(),
                                vars: None
                            })
                        }
                    }
                }

                HostList {
                    hosts: Some(parsed_hosts),
                    groups: self.groups.clone(),
                    vars: self.vars.clone()
                }
            }
            None => {
                HostList {
                    hosts: None,
                    groups: self.groups.clone(),
                    vars: self.vars.clone()
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostList {
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<Host>>,
    pub groups: Option<Vec<Group>>,
}

impl HostList {
    pub fn new() -> HostList {
        HostList {
            vars: Some(HashMap::new()),
            hosts: Some(Vec::new()),
            groups: Some(Vec::new()),
        }
    }

    pub fn from_hosts(hosts: Vec<Host>, vars: Option<HashMap<String, String>>) -> HostList {
        if hosts.is_empty() {
            HostList {
                vars: None,
                hosts: None,
                groups: None
            }
        } else {
            match vars {
                Some(variables) => {
                    HostList {
                        vars: Some(variables),
                        hosts: Some(hosts),
                        groups: None
                    }
                }
                None => {
                    HostList {
                        vars: None,
                        hosts: Some(hosts),
                        groups: None
                    }
                }
            }


        }

    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<String>>
}

// TODO : So far, we assume the hostlist file is in YAML format. More formats will come later.
pub fn hostlist_parser(hostlistcontent: String) -> HostList {

    let parsed_content_temp: HostListVarsUnparsed = serde_yaml::from_str(&hostlistcontent).unwrap();
    let parsed_content = parsed_content_temp.parse();
    parsed_content
}

pub fn hostlist_get_from_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap() // Placeholder : error handling required here
}

pub fn hostlist_get_from_interactive_mode() -> String {
    // Placeholder : we might want a mode where the TaskList is already set and we can add
    // manually / pipe from some other source in real time some more hosts to run the TaskList
    // on and, as soon as the hosts are entered, the TaskList is run on them. Interest ?
    String::new()
}

// TODO : improve clarity ? We are wrapping an Option in a Result. We can have existing empty groups, meaning Ok(None) as a result.
// Interest in allowing empty groups ? To be filled later with interactive mode for example ?
pub fn hostlist_get_specific_group_content(hostlist: &HostList, groupname: &str) -> Result<Option<Vec<String>>, Error> {
    
    match hostlist.groups.clone() {
        Some(groupslist) => {
            match groupslist.iter().position(|group| group.name == groupname) {
                Some(index) => {
                    match groupslist.get(index) {
                        Some(groupcontent) => { return Ok(groupcontent.hosts.clone()) }
                        None => { return Err(Error::FailureToFindGroupContent) }
                    }
                }
                None => { return Err(Error::GroupNotFound) }
            }
        }
        None => { return Err(Error::MissingGroupsList) }
    }
}

pub fn hostlist_get_all_hosts(hostlist: &HostList) -> Option<Vec<String>> {
    
    let mut all_hosts: Vec<Host> = Vec::new();

    match hostlist.clone().hosts {
        Some(hostslist) => {
            all_hosts.extend(hostslist);
        }
        None => {}
    }

    match hostlist.clone().groups {
        Some(groupslist) => {
            for group in groupslist {
                match group.hosts {
                    Some(hostslist) => {
                        let mut hosts: Vec<Host> = Vec::new();
                        for host in hostslist {
                            hosts.push(
                                Host {
                                    address: host,
                                    vars: None
                                }
                            );
                        }
                        all_hosts.extend(hosts);
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
    
    if all_hosts.is_empty() {
        return None;
    } else {
        // TODO : Temporarily still returning Strings as usual, until Host type is used everywhere
        let mut final_hosts_list: Vec<String> = Vec::new();
        for host in all_hosts {
            final_hosts_list.push(host.address);
        }
        Some(final_hosts_list)
    }
}