use serde::Deserialize;
use taskexec::workflow::error::Error;

// TODO : define Host ? any interest ?
// pub struct Host {
//     ...
// }

#[derive(Debug, Deserialize, Clone)]
pub struct HostList {
    pub hosts: Option<Vec<String>>,
    pub groups: Option<Vec<Group>>
}

impl HostList {
    pub fn new() -> HostList {
        HostList {
            hosts: Some(Vec::new()),
            groups: Some(Vec::new())
        }
    }

    pub fn from_hosts(hosts: Vec<String>) -> HostList {
        if hosts.is_empty() {
            HostList {
                hosts: None,
                groups: None
            }
        } else {
            HostList {
                hosts: Some(hosts),
                groups: None
            }
        }

    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub hosts: Option<Vec<String>>
}

// TODO : So far, we assume the hostlist file is in YAML format. More formats will come later.
pub fn hostlist_parser(hostlistcontent: String) -> HostList {

    let parsed_content: HostList = serde_yaml::from_str(&hostlistcontent).unwrap();
    return parsed_content;
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
    
    let mut all_hosts: Vec<String> = Vec::new();

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
                        all_hosts.extend(hostslist);
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
        return Some(all_hosts);
    }
}