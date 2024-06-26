use colored::Colorize;

use termimad::crossterm::style::Color::*;
use termimad::*;
use taskexec::prelude::*;

// Big title : https://patorjk.com/software/taag/#p=display&f=ANSI%20Shadow
// Small title : https://fr.rakko.tools/tools/68/ with 'black_square'

pub fn welcome_message() {

    println!(r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
    🅰🅻🅻-🅸🅽-🅾🅽🅴
");
}

pub fn welcome_message_controller() {

    println!(r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
    🅲🅾🅽🆃🆁🅾🅻🅻🅴🆁
");
}




pub fn welcome_message_worker() {

    println!(r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ 
    🆆🅾🆁🅺🅴🆁                                 
");
}

pub fn welcome_message_agent() {

    println!(r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚═███╔═╝
    ██║  ██║██║   ██║  ███║ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ 
    🅰🅶🅴🅽🆃                                
");
}

// TODO : have this work with an &Assignment instead of an Assignment
pub fn display_output(assignment: Assignment) {

    println!("Host {} : {}", assignment.host.bold(), output_nice_finalstatus(&assignment.finalstatus));

    match assignment.finalstatus {
        AssignmentFinalStatus::Unset => {
            println!("{}", "Assignment is ready to be applied".bold());
        }
        AssignmentFinalStatus::AlreadyMatched => {
            // TODO : more details ?
        }
        AssignmentFinalStatus::FailedDryRun(error) => {
            println!("{}\n", error.red());
            // TODO : show where it failed exactly in the TaskList
        }
        AssignmentFinalStatus::Changed => {
            show_tasklistresult(assignment);
        }
        AssignmentFinalStatus::ChangedWithFailures => {
            show_tasklistresult(assignment);
        }
        AssignmentFinalStatus::FailedChange => {
            show_tasklistresult(assignment);
        }
    }
}

//pub fn display_results_detailed() {}

//pub fn display_results_summary() {}

fn output_nice_result(status: &ApiCallStatus) -> String {
    match status {
        ApiCallStatus::None => { String::from("None") }
        ApiCallStatus::Unset => { String::from("None") }
        ApiCallStatus::ChangeSuccessful(message) => { format!("Success : {}", message) }
        ApiCallStatus::Failure(message) => { format!("Failure : {}", message) }
        ApiCallStatus::AllowedFailure(message) => { format!("Failure (allowed): {}", message) }
    }
}

// TODO : improve this / replace with step name when it will be implemented
fn output_nice_step(step: &Step) -> String {
    match step.name.clone() {
        None => { return format!("`{:?}`", step); }
        Some(content) => { return content; }
    }
    
}

fn output_nice_finalstatus(finalstatus: &AssignmentFinalStatus) -> String {
    match finalstatus {
        AssignmentFinalStatus::Unset => {
            return format!("{}", "Unset".red().bold()); // Should never occur
        }
        AssignmentFinalStatus::FailedDryRun(_error) => {
            return format!("{}", "Failed dry run".red().bold());
        }
        AssignmentFinalStatus::Changed => {
            return format!("{}", "Changed".blue().bold());
        }
        AssignmentFinalStatus::ChangedWithFailures => {
            return format!("{}", "Changed (with failures)".truecolor(255, 90, 0).bold());
        }
        AssignmentFinalStatus::FailedChange => {
            return format!("{}", "Failed change".red().bold());
        }
        AssignmentFinalStatus::AlreadyMatched => {
            return format!("{}", "Matched".green().bold());
        }
    }
}

fn show_tasklistresult(assignment: Assignment) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(White);

    // 1 display per Task
    for taskindex in 0..assignment.tasklistresult.taskresults.len() {

        match &assignment.tasklistresult.taskresults[taskindex].stepresults {
            None => {
                println!("Task : {} -> {}",
                    &assignment.tasklist.tasks[taskindex].name.clone().unwrap_or(String::from("no name for TaskBlock")).bold(),
                    "no result".bold()
                );
            }
            Some(stepresults) => {
                println!("Task : {}",
                &assignment.tasklist.tasks[taskindex].name.clone().unwrap_or(String::from("no name for TaskBlock")).bold()
                );

                let mut table_content = String::new();
                table_content.push_str("|:-:|:-:|-");
                table_content.push_str("\n|**Step**|**Changes**|**Results**|");
                table_content.push_str("\n|-");

                for (stepindex, stepresultcontent) in stepresults.iter().enumerate() {
                    // One step can represent multiple changes so the 1st line is displayed by itself, with the name
                    // of the step, then the rest without this name
                            
                    table_content.push_str(
                        format!("\n|{}|{}|{}|",
                            output_nice_step(&assignment.tasklist.tasks[taskindex].steps[stepindex]),
                            assignment.changelist.taskchanges.clone().unwrap()[taskindex].stepchanges.clone()[stepindex].display()[0],
                            output_nice_result(&assignment.tasklistresult.clone().taskresults[taskindex].stepresults.clone().unwrap()[stepindex].apicallresults[0].status)
                        ).as_str()
                    );

                    for (apicallindex, _apicallcontent) in stepresultcontent.apicallresults.iter().enumerate() {
                        if apicallindex > 0 {
                            table_content.push_str(
                                format!("\n||{}|{}|",
                                    assignment.changelist.taskchanges.clone().unwrap()[taskindex].stepchanges.clone()[stepindex].display()[apicallindex],
                                    output_nice_result(&assignment.tasklistresult.clone().taskresults[taskindex].stepresults.clone().unwrap()[stepindex].apicallresults[apicallindex].status)
                                ).as_str()
                            );
                        }
                    }


    //                         match &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[0].status {
    //                             ApiCallStatus::Failure(_) => {
    //                                 // Stop the table and show the full detail
    //                                 table_content.push_str("\n|-");
    //                                 println!("{}", skin.term_text(&table_content));
    //                                 println!("{}",
    //                                     &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[0].output.as_ref().unwrap()
    //                                         .red().bold()
    //                                     );
                                    
    //                                 return;
    //                             }
    //                             _ => {}
    //                         }
                }

                // Close the table and display it
                table_content.push_str("\n|-");
                println!("{}", skin.term_text(&table_content));

                // If the last result is a Failure, display details about it
                match &stepresults.last().unwrap().apicallresults.last().unwrap().status {
                    ApiCallStatus::Failure(_) => {
                        println!("{}",
                            &stepresults.last().unwrap().apicallresults.last().unwrap().output.as_ref().unwrap().red()
                        );
                    }
                    _ => {}
                }
            }
        }
    }


    // for (taskblockindex, taskblock) in assignment.tasklist.tasks.iter().enumerate() {
    //     table_content.push_str(format!("\nTask : *{}*", taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock"))).as_str());
    //     table_content.push_str("\n|:-:|:-:|-");
    //     table_content.push_str("\n|**Step**|**Changes**|**Results**|");
    //     for (stepindex, step) in taskblock.steps.iter().enumerate() {
    //         table_content.push_str("\n|:-|:-:|-");
            
    //         match &assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges[stepindex] {
    //             ModuleBlockChange::AlreadyMatched(message) => {
    //                 table_content.push_str(
    //                     format!("\n|{}| Matched : {}|{}|",
    //                             output_nice_step(&step),
    //                             message,
    //                             "N/A"
    //                         ).as_str()
    //                 );
    //             }
    //             ModuleBlockChange::FailedToEvaluate(message) => {
    //                 table_content.push_str(
    //                     format!("\n|{}| Failed to evaluate : {}|{}|",
    //                             output_nice_step(&step),
    //                             message,
    //                             "N/A"
    //                         ).as_str()
    //                 );
    //             }
    //             ModuleBlockChange::ModuleApiCalls(apicalls) => {

    //                 for taskresultindex in 0..assignment.tasklistresult.taskresults.len() {
    //                     for stepresultindex in 0..assignment.tasklistresult.taskresults[taskresultindex].stepresults.as_ref().unwrap().len() {
                            
    //                         table_content.push_str(
    //                             format!("\n|{}|{}|{}|",
    //                                 output_nice_step(&step),
    //                                 assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepresultindex].display()[0],
    //                                 output_nice_result(&assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[0].status)
    //                             ).as_str()
    //                         );

    //                         match &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[0].status {
    //                             ApiCallStatus::Failure(_) => {
    //                                 // Stop the table and show the full detail
    //                                 table_content.push_str("\n|-");
    //                                 println!("{}", skin.term_text(&table_content));
    //                                 println!("{}",
    //                                     &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[0].output.as_ref().unwrap()
    //                                         .red().bold()
    //                                     );
                                    
    //                                 return;
    //                             }
    //                             _ => {}
    //                         }
                            
                            // for (apicallindex, _apicallcontent) in apicalls.iter().enumerate() {
                            //     if apicallindex > 0 {
                            //         table_content.push_str(
                            //             format!("\n||{}|{}|",
                            //                 assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepresultindex].display()[apicallindex],
                            //                 output_nice_result(&assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[apicallindex].status)
                            //             ).as_str()
                            //         );

                            //         match &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[apicallindex].status {
                            //             ApiCallStatus::Failure(_) => {
                            //                 // Stop the table and show the full detail
                            //                 table_content.push_str("\n|-");
                            //                 println!("{}", skin.term_text(&table_content));
                            //                 println!("{}",
                            //                     &assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepresultindex].apicallresults[apicallindex].output.as_ref().unwrap()
                            //                         .red().bold()
                            //                     );
                                            
                            //                 return;
                            //             }
                            //             _ => {}
                            //         }
                            //     }
                            // }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // table_content.push_str("\n|-");
    // println!("{}", skin.term_text(&table_content));
}