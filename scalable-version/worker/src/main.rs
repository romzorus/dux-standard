use cli::prelude::*;
use connection::prelude::*;
use std::sync::Mutex;
use taskexec::prelude::*;

fn main() {

    welcome_message();
    println!("[Dux worker]"); // TODO : have a nice display for this also
    println!("");

    // Parse the CLI arguments
    let cliargs: CliArgs = parse_cli_args();

    // "Consume" Assignments from the Message Broker (MB)
    let mut assignmentlist: Vec<Assignment> = Vec::new();

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
                    if let AssignmentFinalStatus::Unset = assignment.finalstatus {
                            assignment.apply();
                    }
                    resultslist.lock().unwrap().push(assignment);
                });
            }
        });
    });
    
    // "Produce" Assignments (with DryRun and Results filled) and send them to the MB

}
