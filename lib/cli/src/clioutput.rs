use taskexec::workflow::result::TaskListResult;

pub fn display_output(allresults: Vec<TaskListResult>) {

    for tasklistresult in allresults {
        println!("###### {} ######", tasklistresult.host);

        for taskresult in tasklistresult.results {
            // TODO :Get the name of the Task in TaskResult so it can be displayed here

            match taskresult.list {
                None => { println!("-> No result for this Task") }
                Some(moduleblockresultlist) => {

                    for moduleblockresult in moduleblockresultlist {
                        // TODO : Get the name of the module so it can be displayed here
                        match moduleblockresult.exitcode {
                            None => { println!("No exitcode : something went wrong"); }
                            Some(exitcode) => {
                                if exitcode == 0 {
                                    println!("Success");
                                } else {
                                    println!("Failure : exitcode = {}", moduleblockresult.exitcode.unwrap());
                                }
                            }
                        }

                    }

                }
            }

        }
    }

    // println!("**** Host : {} *****", execresult.host);
    // for result in execresult.clone().results.into_iter() {
    //     for blockresult in result.list.into_iter() {
    //         for moduleblockresult in blockresult.into_iter() {
    //             if let Some(content) = moduleblockresult.stdout {
    //                 println!("{}", content.trim());
    //             }
    //         }
    //     }
    // }

}

pub fn display_results_detailed() {}

pub fn display_results_summary() {}