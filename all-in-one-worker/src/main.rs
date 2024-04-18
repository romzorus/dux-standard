use cli::prelude::*;
use connection::prelude::*;
use hostparser::*;
use std::{path::PathBuf, sync::Mutex};
use taskexec::prelude::*;
use taskparser::prelude::*;

fn main() {
    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // Build a TaskList (YAML is assumed for now)
    let tasklist = tasklist_parser(
        tasklist_get_from_file(&cliargs.tasklist)
        );

    // Build a HostList (not implemented yet)
    let hostlist = hostlist_parser(
        hostlist_get_from_file(&cliargs.hostlist)
    );

    // Build Assignments (an Assignment is basically a Host associated to a TaskList)
    //  -> Initialization of CorrelationId (not really required for all-in-one mode)
    let mut correlationid = CorrelationIdGenerator::new();
    correlationid.init();
    //  -> Actual build of Assignments
    let mut assignmentlist: Vec<Assignment> = Vec::new();

    for host in hostlist_get_all_hosts(&hostlist).unwrap() {

        let mut hosthandler = HostHandler::from(ConnectionMode::Ssh2, host.clone());

        match &cliargs.key {
            Some(privatekeypath) => {
                hosthandler.ssh2auth(
                    Ssh2AuthMode::SshKeys((
                        cliargs.user.clone(),
                        PathBuf::from(privatekeypath)
                    ))
                );
            }
            None => {
                // No SSH key given as argument, trying with password if it is given
                match cliargs.password.clone() {
                    Some(pwd) => {
                        hosthandler.ssh2auth(
                            Ssh2AuthMode::UsernamePassword(
                                Credentials::from(cliargs.user.clone(), pwd)
                            )
                        );
                    }
                    None => {
                        println!("No SSH key or password to connect to remote host.")
                    }
                }
            }
        }
        
        assignmentlist.push(Assignment::from(
            correlationid.get_new_value().unwrap(),
            RunningMode::Apply,
            host.clone(),
            hosthandler,
            tasklist.clone(),
            ChangeList::new(),
            TaskListResult::new(),
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
                    let _ = assignment.dry_run();
                    assignment.apply();
                    resultslist.lock().unwrap().push(assignment);
                });
            }
        });
    });
    
    // TODO : implement a better way to sort the output according to the order of the hosts in the HostList
    // aka sort resultslist in HostList order so we simply have to go through resultslist after that
    for host in hostlist_get_all_hosts(&hostlist).unwrap() {
        for assignment in resultslist.lock().unwrap().clone().into_iter() {
            if host.eq(&assignment.host) {
                display_output(assignment);
            }
        }
    }
}
