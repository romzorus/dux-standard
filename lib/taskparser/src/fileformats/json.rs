use taskexec::workflow::task::{Task, TaskList};
use errors::Error;
use serde_json;

pub fn json_tasklist_parser(tasklistcontent: &String) -> Result<TaskList, Error> {

    match serde_json::from_str::<Vec<Task>>(tasklistcontent) {
        Ok(parsed_content) => {
            return Ok(TaskList::from(parsed_content));
        }
        Err(_e) => {
            return Err(Error::FailureToParseFile)
        }
    }
}
