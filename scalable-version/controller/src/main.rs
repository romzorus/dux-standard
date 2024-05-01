// The controller is handling :
    // - CLI
    // - TaskList parsing
    // - HostList parsing
    // - Assignments production
    // - Results display

use cli::prelude::*;
use connection::prelude::*;
use hostparser::*;
use taskexec::prelude::*;
use taskparser::prelude::*;
use std::{path::PathBuf, sync::Mutex};

fn main() {
    welcome_message();
    println!("[Dux controller]"); // TODO : have a nice display for this also
    println!("");

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
 
    let resultslist: Vec<Assignment> = Vec::new();

    // Now, Assignments need to be sent to Message Broker (MB).
    // After this, "consume" the results from MB and push them in 'resultslist'.
    // To make sure we have all results, establish a list of CorrelationId and
    // use it as a checklist.

    // resultslist.push(assignment);

    
    // TODO : implement a better way to sort the output according to the order of the hosts in the HostList
    // aka sort resultslist in HostList order so we simply have to go through resultslist after that
    for host in hostlist_get_all_hosts(&hostlist).unwrap() {
        for assignment in resultslist.clone().into_iter() {
            if host.eq(&assignment.host) {
                display_output(assignment);
            }
        }
    }
}
