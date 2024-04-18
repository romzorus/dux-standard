use termimad::crossterm::style::Color::*;
use termimad::*;
use taskexec::prelude::*;

pub fn welcome_message() {

    println!(r"
    ██████╗ ██╗   ██╗██╗  ██╗
    ██╔══██╗██║   ██║╚██╗██╔╝
    ██║  ██║██║   ██║ ╚███╔╝ 
    ██║  ██║██║   ██║ ██╔██╗ 
    ██████╔╝╚██████╔╝██╔╝ ██╗
    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝
");
}

pub fn display_output(assignment: Assignment) {

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(White);

    let mut table_content = String::new();

    table_content.push_str(format!("Host : ***{}*** ({})", assignment.host, output_nice_finalstatus(&assignment.finalstatus)).as_str());

    match assignment.finalstatus {
        // AssignmentFinalStatus::Unset => {}
        AssignmentFinalStatus::Failed(error) => {
            table_content.push_str("\n|-:|:-:|");
            table_content.push_str(
                format!("\n|**Failed**|{}|", error).as_str()
            );
        }
        _ => {
            table_content.push_str("\n|-:|:-:|:-:|-");
            table_content.push_str("\n|**Task**|**Step**|**Changes**|**Results**|");
           
            for (taskblockindex, taskblock) in assignment.tasklist.tasks.iter().enumerate() {
                // A "step" is a ModuleBlockExpectedState (simplifies the reading)
                for (stepindex, step) in taskblock.steps.iter().enumerate() {
                    table_content.push_str("\n|-:|:-|:-:|-");
                    
                    match &assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges[stepindex] {
                        ModuleBlockChange::AlreadyMatched(message) => {
                            table_content.push_str(
                                format!("\n|{}|{}| Matched : {}|{}|",
                                        taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock")),
                                        output_nice_step(&step),
                                        message,
                                        "N/A"
                                    ).as_str()
                            );
                        }
                        ModuleBlockChange::FailedToEvaluate(message) => {
                            table_content.push_str(
                                format!("\n|{}|{}| Failed to evaluate : {}|{}|",
                                        taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock")),
                                        output_nice_step(&step),
                                        message,
                                        "N/A"
                                    ).as_str()
                            );
                        }
                        ModuleBlockChange::ModuleApiCalls(apicalls) => {
                            table_content.push_str(
                                format!("\n|{}|{}|{}|{}|",
                                    taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock")),
                                    output_nice_step(&step),
                                    assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepindex].display()[0],
                                    output_nice_result(&assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepindex].apicallresults[0].status)
                                ).as_str()
                            );
                            
                            for (apicallindex, apicallcontent) in apicalls.iter().enumerate() {
                                if apicallindex > 0 {
                                    table_content.push_str(
                                        format!("\n|||{}|{}|",
                                            assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepindex].display()[apicallindex],
                                            output_nice_result(&assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepindex].apicallresults[apicallindex].status)
                                        ).as_str()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    table_content.push_str("\n|-");
    println!("{}", skin.term_text(&table_content));
}

//pub fn display_results_detailed() {}

//pub fn display_results_summary() {}

fn output_nice_result(status: &ApiCallStatus) -> String {
    match status {
        ApiCallStatus::None => { String::from("None") }
        ApiCallStatus::Unset => { String::from("None") }
        ApiCallStatus::ChangeSuccessful(message) => { format!("Success : {}", message) }
        ApiCallStatus::ChangeFailed(message) => { format!("Failure : {}", message) }
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
            return String::from("Unset");
        }
        AssignmentFinalStatus::Failed(_error) => {
            return format!("Failed");
        }
        AssignmentFinalStatus::Changed => {
            return String::from("Changed");
        }
        AssignmentFinalStatus::AlreadyMatched => {
            return String::from("Matched");
        }
    }
}