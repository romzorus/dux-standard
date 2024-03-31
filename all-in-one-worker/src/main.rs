use cliparser::{parse_cli_args, CliArgs};
use connection::prelude::*;
use hostparser::*;
use std::path::PathBuf;
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
    //  -> Initialization of CorrelationId
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
            tasklist.clone(),
            hosthandler
        ));
    }

    // Run the Assignments and save the Results
    //  -> Open a vector to put the results in
    let mut results: Vec<TaskListResult> = Vec::new();
    //  -> Run each Assignment
    for mut assignment in assignmentlist.into_iter() {

        println!("**** Host : {} *****", assignment.host);
        let execresult = assignment.dry_run().apply_changelist(&mut assignment.hosthandler);
        
        for result in execresult.clone().results.into_iter() {
            for blockresult in result.list.into_iter() {
                for moduleblockresult in blockresult.into_iter() {
                    if let Some(content) = moduleblockresult.stdout {
                        println!("{}", content.trim());
                    }
                }
            }
        }

        results.push(execresult);
    }
    // results now contains all the output of the run, per host
}
