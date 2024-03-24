use taskexec::prelude::*;
use taskparser::prelude::*;
use cliparser::{parse_cli_args, CliArgs };
use hostparser::*;

fn main() {
    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // Build a TaskList (YAML is assumed for now)
    let tasklist = tasklist_parser(
        tasklist_get_from_file(&cliargs.tasklist),
        ContentFormat::Yaml);
    
    // Build a HostList (not implemented yet)
    let hostlistcontent = hostlist_get_from_file("examples/hostlists/hostlist");
    let hostlist = hostlist_parser(hostlistcontent);

    // Build Assignments (an Assignment is basicall a Host associated to a TaskList)
    //  -> Initialization of CorrelationId
    let mut correlationid = CorrelationIdGenerator::new();
    correlationid.init();
    //  -> Actual build of Assignments
    let mut assignmentlist: Vec<Assignment> = Vec::new();

    for host in hostlist_get_all_hosts(&hostlist).unwrap() {
        assignmentlist.push(
            Assignment::from(
                correlationid.get_new_value().unwrap(),
                RunningMode::Apply,
                host,
                tasklist.clone())
        );
    }

    // Run the Assignments and save the Results
    //  -> Open a vector to put the results in
    let mut results: Vec<TaskListResult> = Vec::new();
    //  -> Run each Assignment
    for assignment in assignmentlist.into_iter() {
        let execresult = assignment.dry_run().apply_changelist();
        
        println!("**** Host : {} *****", assignment.host);
        println!("{:?}", execresult);

        results.push(execresult);
    }
    // results now contains all the output of the run, per host
}