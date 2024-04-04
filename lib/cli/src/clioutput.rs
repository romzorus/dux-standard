use taskexec::{prelude::*, workflow::assignment};

pub fn display_output(assignment: Assignment) {

    for (taskblockindex, taskblock) in assignment.tasklist.tasks.iter().enumerate() {
        for (stepindex, step) in taskblock.steps.iter().enumerate() {
            println!("############ {} ############", assignment.host);
            for changeresult in assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepindex].apicallresults.iter() {
                println!("{} |||| {:?} |||| {:?} |||| {:?}",
                    taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock")),
                    step,
                    assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepindex],
                    changeresult.status
                );
            }
        }
    }
}

pub fn display_results_detailed() {}

pub fn display_results_summary() {}

pub fn display_matrix() {
    // for row in matrix {
    //     println!("{} - {} - {} - {} - {:?} - {:?}",
    //         row.taskid,
    //         row.taskname,
    //         row.blockid,
    //         row.blockcontentshort,
    //         row.changes,
    //         row.results
    //     );
    // }
}