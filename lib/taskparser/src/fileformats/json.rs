use taskexec::workflow::task::{TaskBlock, TaskList};
use errors::Error;
use serde_json;

pub fn json_tasklist_parser(tasklistcontent: &String) -> Result<TaskList, Error> {

    match serde_json::from_str::<Vec<TaskBlock>>(tasklistcontent) {
        Ok(parsed_content) => {
            return Ok(TaskList::from(parsed_content));
        }
        Err(e) => {
            return Err(Error::FailureToParseFile(format!("{:?}", e)))
        }
    }
}
