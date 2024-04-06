use termimad::crossterm::style::Color::*;
use termimad::*;
use taskexec::{prelude::*, workflow::assignment};

pub fn display_output(assignment: Assignment) {

    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(White);

    let mut table_content = String::new();

    table_content.push_str(format!("Host : ***{}***", assignment.host).as_str());

    table_content.push_str("\n|-:|:-:|:-:|-");
    table_content.push_str("\n|**Task**|**Step**|**Changes**|**Results**|");

    for (taskblockindex, taskblock) in assignment.tasklist.tasks.iter().enumerate() {
        for (stepindex, step) in taskblock.steps.iter().enumerate() {
            for changeresult in assignment.tasklistresult.clone().taskresults[taskblockindex].stepresults.clone().unwrap()[stepindex].apicallresults.iter() {
                
                table_content.push_str("\n|-:|:-|:-:|-");

                let step_content = format!("{:#?}", step);
                let lines_in_step: Vec<&str> = step_content.lines().collect();

                for (linenumber, linecontent) in lines_in_step.iter().enumerate() {
                    if linenumber == 0 {
                        table_content.push_str(
                            format!("\n|{}|{}|{}|{}|",
                                taskblock.name.clone().unwrap_or(String::from("no name for TaskBlock")),
                                linecontent,
                                assignment.changelist.taskchanges.clone().unwrap()[taskblockindex].stepchanges.clone()[stepindex].display()[0],
                                output_nice_result(&changeresult.status)
                            ).as_str()
                        );
                    } else {
                        table_content.push_str(
                            format!("\n||{}|||", linecontent).as_str()
                        );
                    }
                }
            }
        }
    }

    table_content.push_str("\n|-");
    println!("{}", skin.term_text(&table_content));
    println!("\n");
}

pub fn display_results_detailed() {}

pub fn display_results_summary() {}


fn output_nice_result(status: &ApiCallStatus) -> String {
    match status {
        ApiCallStatus::None => { String::from("None") }
        ApiCallStatus::Unset => { String::from("None") }
        ApiCallStatus::ChangeSuccessful(message) => { format!("Success : {}", message) }
        ApiCallStatus::ChangeFailed(message) => { format!("Failure : {}", message) }
    }
}