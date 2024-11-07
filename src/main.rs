use std::process::exit;

use duxcore::prelude::*;

mod cliargs;
mod conf;

use crate::cliargs::{parse_cli_args_standard, CliArgsStandard};
use crate::conf::DuxConfigStandard;

fn main() {

    // Parse the CLI arguments
    let cliargs: CliArgsStandard = parse_cli_args_standard().unwrap();

    // Get the configuration (not used for now)
    let _conf =
        DuxConfigStandard::from(cliargs.conf).expect("Unable to determine configuration. Abort.");

    // Build a HostList
    let host_list = match &cliargs.hostlist {
        Some(hostlist_path) => match HostList::from_file(&hostlist_path) {
            Ok(hostlist) => hostlist,
            Err(error) => {
                println!("Unable to read hoslist file. Abort.");
                println!("{:?}", error);
                exit(1);
            }
        },
        None => {
            println!("No hostlist path provided. Abort.");
            exit(1);
        }
    };

    // If hostlist is empty, stop there
    if hostlist_get_all_hosts(&host_list).is_none() {
        println!("No hosts in given list ({})", cliargs.hostlist.unwrap());
        exit(0);
    }

    // How do we connect to all hosts ?
    let host_connection_info = match &cliargs.key {
        Some(privatekeypath) => {
            HostConnectionInfo::ssh2_with_key_file(&cliargs.user.unwrap(), privatekeypath)
        }
        None => {
            // No SSH key given as argument, trying with password if it is given
            match cliargs.password.clone() {
                Some(pwd) => {
                    HostConnectionInfo::ssh2_with_username_password(cliargs.user.unwrap(), pwd)
                }
                None => {
                    panic!("No SSH key or password to connect to remote host.");
                    // TODO : gracefully quit instead of panic
                }
            }
        }
    };

    // Build a JobList based on all previous work (HostList, TaskList, CLI arguments and so on...)
    let mut job_list = JobList::from_hostlist(host_list);

    job_list
        .set_connection(host_connection_info)
        .unwrap()
        .set_tasklist_from_file(&cliargs.tasklist.unwrap(), TaskListFileType::Unknown)
        .unwrap();

    job_list.apply();

    println!("{}", job_list.display_pretty());
}

