use std::collections::HashMap;
use std::process::exit;
use duxcore::cli::args::versions::agent::CliArgsAgent;
use duxcore::prelude::*;

fn main() {

    welcome_message_agent();

    let cliargs: CliArgsAgent = parse_cli_args_agent().unwrap();

    let tasklist = tasklist_parser(
        tasklist_get_from_file(&cliargs.tasklist.as_ref().unwrap()),
        &Host::from_string("localhost".to_string())
        );

    if tasklist.tasks.is_empty() {
        println!("No task in given list ({})", &cliargs.tasklist.unwrap());
        exit(0);
    }

    let mut correlationid = CorrelationIdGenerator::new();
    match correlationid.init() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: failure to initialize CorrelationId");
            println!("{:?}", e);
            exit(1);
        }
    }

    let mut assignment = Assignment::from(
        correlationid.get_new_value().unwrap(), // This unwrap() is safe because initialization is checked before.
        RunningMode::Apply,
        "localhost".to_string(),
        HostHandlingInfo::from(ConnectionMode::LocalHost, "localhost".to_string(), ConnectionDetails::LocalHost(LocalHostConnectionDetails::current_user())),
        HashMap::new(),
        tasklist.clone(),
        ChangeList::new(),
        ResultList::new(),
        AssignmentFinalStatus::Unset
    );


    let mut hosthandler = HostHandler::from(&assignment.hosthandlinginfo).unwrap();

    let _ = hosthandler.init();

    let _ = assignment.dry_run(&mut hosthandler);
    if let AssignmentFinalStatus::Unset = assignment.finalstatus {
            assignment.apply(&mut hosthandler);
    }

    display_output(assignment);
}
