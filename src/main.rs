use std::collections::HashMap;
use std::{path::PathBuf, process::exit, sync::Mutex};

use duxcore::prelude::*;

mod cliargs;
mod conf;

use crate::cliargs::{parse_cli_args_standard, CliArgsStandard};
use crate::conf::DuxConfigStandard;

fn main() {

    welcome_message_standard();

    // Parse the CLI arguments
    let cliargs: CliArgsStandard = parse_cli_args_standard().unwrap();

    // Get the configuration (not used for now)
    let _conf = DuxConfigStandard::from(cliargs.conf).expect("Unable to determine configuration. Abort.");
    
    // Build a HostList
    let hostlist = hostlist_parser(
        hostlist_get_from_file(&cliargs.hostlist.as_ref().unwrap())
    );

    if hostlist_get_all_hosts(&hostlist).is_none() {
        println!("No hosts in given list ({})", &cliargs.hostlist.unwrap());
        exit(0);
    }

    // Build Assignments (an Assignment is basically a Host associated to a TaskList)
    //  -> Initialization of CorrelationId (not really required for all-in-one mode)
    let mut correlationid = CorrelationIdGenerator::new();
    match correlationid.init() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: failure to initialize CorrelationId");
            println!("{:?}", e);
            exit(1);
        }
    }
    //  -> Actual build of Assignments
    let mut assignmentlist: Vec<Assignment> = Vec::new();

    // This unwrap is safe since we checked before that the list is not empty.
    //for host in hostlist_get_all_hosts(&hostlist).unwrap() {
    for host in hostlist.hosts.as_ref().unwrap() {
    
        let authmode = match &cliargs.key {
            Some(privatekeypath) => {
                Ssh2AuthMode::SshKeys((
                    cliargs.user.clone().unwrap(),
                    PathBuf::from(privatekeypath)
                ))
            }
            None => {
                // No SSH key given as argument, trying with password if it is given
                match cliargs.password.clone() {
                    Some(pwd) => {
                        Ssh2AuthMode::UsernamePassword(
                            Credentials::from(cliargs.user.clone().unwrap(), pwd)
                        )
                    }
                    None => {
                        panic!("No SSH key or password to connect to remote host."); // TODO : gracefully quit instead of panic
                    }
                }
            }
        };

        let tasklist = tasklist_parser(
            tasklist_get_from_file(&cliargs.tasklist.as_ref().unwrap()),
            &host
            );
    
        if tasklist.tasks.is_empty() {
            println!("No task in given list ({})", &cliargs.tasklist.unwrap());
            exit(0);
        }

        // SSH2 is assumed for now.
        assignmentlist.push(Assignment::from(
            correlationid.get_new_value().unwrap(), // This unwrap() is safe because initialization is checked before.
            RunningMode::Apply,
            host.address.clone(),
            HostHandlingInfo::from(ConnectionMode::Ssh2, host.address.clone(), ConnectionDetails::Ssh2(Ssh2ConnectionDetails::from(host.address.clone(), authmode))),
            HashMap::new(),
            tasklist.clone(),
            ChangeList::new(),
            ResultList::new(),
            AssignmentFinalStatus::Unset
        ));
    }

    // If the number of threads to use is not specified, one thread per CPU of the local machine
    let threads_number = match cliargs.threads {
        None => { std::thread::available_parallelism().unwrap().get() }
        Some(number) => { number }
    };
 
    let resultslist: Mutex<Vec<Assignment>> = Mutex::new(Vec::new());

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads_number)
        .build()
        .unwrap();

    pool.install(|| {
        rayon::scope(|s| {
            for mut assignment in assignmentlist.into_iter() {
                let resultslist = &resultslist;
                s.spawn(move |_| {

                    let mut hosthandler = HostHandler::from(&assignment.hosthandlinginfo).unwrap();

                    let _ = hosthandler.init();

                    let _ = assignment.dry_run(&mut hosthandler);
                    if let AssignmentFinalStatus::Unset = assignment.finalstatus {
                            assignment.apply(&mut hosthandler);
                    }
                    resultslist.lock().unwrap().push(assignment);
                });
            }
        });
    });
    
    // TODO : implement a better way to sort the output according to the order of the hosts in the HostList
    // aka sort resultslist in HostList order so we simply have to go through resultslist after that
    for host in hostlist.hosts.unwrap() {
        for assignment in resultslist.lock().unwrap().clone().into_iter() {
            if host.address.eq(&assignment.host) {
                display_output(assignment);
            }
        }
    }
}

pub fn welcome_message_standard() {
    println!(
        r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
    🆂🆃🅰🅽🅳🅰🆁🅳
"
    );
}
