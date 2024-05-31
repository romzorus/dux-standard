use std::collections::HashMap;
use serde::Deserialize;
use errors::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Host {
    address: String,
    vars: Option<HashMap<String, String>>,
    groups: Option<Vec<String>>
}

impl Host {
    pub fn from_string(address: String) -> Host {
        Host {
            address,
            vars: None,
            groups: None
        }
    }

    pub fn add_to_group(&mut self, groupname: &String) {
        match &self.groups {
            Some(group_list) => {
                let mut new_group_list = group_list.clone();
                new_group_list.push(groupname.clone());
                self.groups = Some(new_group_list);
            }
            None => {
                self.groups = Some(vec![groupname.clone()]);
            }
        }
    }

    pub fn add_vars(&mut self, newvars: &HashMap<String, String>) {
        match &self.vars {
            Some(oldvars) => {
                let mut new_vars_list = oldvars.clone();
                new_vars_list.extend(newvars.clone());
                self.vars = Some(new_vars_list);
            }
            None => {
                self.vars = Some(newvars.clone());
            }
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
    pub fn parse_host_vars(&self) -> HostListFile {

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
                                vars: Some(vars_list),
                                groups: None
                            })
                        }
                        None => {
                            parsed_hosts.push(Host {
                                address: hostname.to_string(),
                                vars: None,
                                groups: None
                            })
                        }
                    }
                }

                HostListFile {
                    hosts: Some(parsed_hosts),
                    groups: self.groups.clone(),
                    vars: self.vars.clone()
                }
            }
            None => {
                HostListFile {
                    hosts: None,
                    groups: self.groups.clone(),
                    vars: self.vars.clone()
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostListFile {
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<Host>>,
    pub groups: Option<Vec<Group>>,
}

impl HostListFile {
    pub fn new() -> HostListFile {
        HostListFile {
            vars: Some(HashMap::new()),
            hosts: Some(Vec::new()),
            groups: Some(Vec::new()),
        }
    }

    pub fn from_hosts(hosts: Vec<Host>, vars: Option<HashMap<String, String>>) -> HostListFile {
        if hosts.is_empty() {
            HostListFile {
                vars: None,
                hosts: None,
                groups: None
            }
        } else {
            match vars {
                Some(variables) => {
                    HostListFile {
                        vars: Some(variables),
                        hosts: Some(hosts),
                        groups: None
                    }
                }
                None => {
                    HostListFile {
                        vars: None,
                        hosts: Some(hosts),
                        groups: None
                    }
                }
            }


        }
    }

    // This method will gather all elements about a host and create a Host object / per host
    // -> address + all variables which applies to this host + all groups this host belongs to
    pub fn generate_hostlist(&self) -> HostList {
        let mut final_hostlist: Vec<Host> = Vec::new();

        match &self.groups {
            Some(groups_list) => {
                for group in groups_list {
                    match &group.hosts {
                        Some(host_list) => {
                            for host_address in host_list {
                                match find_host_in_list(&final_hostlist, &host_address) {
                                    Some(index) => {
                                        final_hostlist[index].add_to_group(&group.name);
                                        // Only add group level variables because the host is already in the list, meaning it already has HostList level variables
                                        final_hostlist[index].add_vars(&group.vars.as_ref().unwrap());
                                    }
                                    None => {
                                        let mut temp_host = Host::from_string(host_address.clone());
                                        temp_host.add_to_group(&group.name);
                                        // First, add HostList level variables
                                        temp_host.add_vars(&self.vars.as_ref().unwrap());
                                        // Then add group level variables (surcharge)
                                        temp_host.add_vars(&group.vars.as_ref().unwrap());
        
                                        final_hostlist.push(temp_host);
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        match &self.hosts {
            Some(host_list) => {
                for host in host_list {
                    match find_host_in_list(&final_hostlist, &host.address) {
                        Some(index) => {
                            // Host is already part of a group, only host vars need to be added
                            final_hostlist[index].add_vars(host.vars.as_ref().unwrap());
                        }
                        None => {
                            let mut temp_host = Host::from_string(host.address.clone());
                            // First, add HostList level variables
                            temp_host.add_vars(&self.vars.as_ref().unwrap());
                            // Then add host level variables (surcharge)
                            temp_host.add_vars(&host.vars.as_ref().unwrap());

                            final_hostlist.push(temp_host);
                        }
                    }
                }
            }
            None => {}
        }

        if final_hostlist.is_empty() {
            HostList { hosts: None}
        } else {
            HostList { hosts: Some(final_hostlist) }
        }
    }
}

// If the host is already in the list, the index is returned. Otherwise, None is returned.
fn find_host_in_list(hosts_list: &Vec<Host>, host_name: &String) -> Option<usize> {
    for (index, host) in hosts_list.iter().enumerate() {
        if host.address.eq(host_name) {
            return Some(index);
        }
    }
    None
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
    pub hosts: Option<Vec<String>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct HostList {
    pub hosts: Option<Vec<Host>>
}

// TODO : So far, we assume the hostlist file is in YAML format. More formats will come later.
pub fn hostlist_parser(hostlistfilecontent: String) -> HostList {
    // First we parse the content as YAML, host vars not parsed yet (unproper YAML syntax)
    let yaml_parsed_result: HostListVarsUnparsed = serde_yaml::from_str(&hostlistfilecontent).unwrap();
    // Second we parse the host vars
    let host_vars_parsed_result = yaml_parsed_result.parse_host_vars();
    // Finally, we generate a HostList out of the HostListFile
    host_vars_parsed_result.generate_hostlist()
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
// pub fn hostlist_get_specific_group_content(hostlist: &HostList, groupname: &str) -> Result<Option<Vec<String>>, Error> {
    
//     match hostlist.groups.clone() {
//         Some(groupslist) => {
//             match groupslist.iter().position(|group| group.name == groupname) {
//                 Some(index) => {
//                     match groupslist.get(index) {
//                         Some(groupcontent) => { return Ok(groupcontent.hosts.clone()) }
//                         None => { return Err(Error::FailureToFindGroupContent) }
//                     }
//                 }
//                 None => { return Err(Error::GroupNotFound) }
//             }
//         }
//         None => { return Err(Error::MissingGroupsList) }
//     }
// }

pub fn hostlist_get_all_hosts(hostlist: &HostList) -> Option<Vec<String>> {

    match &hostlist.hosts {
        Some(host_list) => {
            let mut all_hosts_addresses: Vec<String> = Vec::new();
            for host in host_list {
                all_hosts_addresses.push(host.address.clone());
            }
            Some(all_hosts_addresses)
        }
        None => {
            None
        }
    }

}