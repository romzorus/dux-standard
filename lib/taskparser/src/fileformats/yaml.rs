use taskexec::workflow::task::{TaskBlock, TaskList};
use errors::Error;
use serde_yaml;

pub fn yaml_tasklist_parser(tasklistcontent: &String) -> Result<TaskList, Error> {

    match serde_yaml::from_str::<Vec<TaskBlock>>(tasklistcontent) {
        Ok(parsed_content) => {
            return Ok(TaskList::from(parsed_content));
        }
        Err(_e) => {
            return Err(Error::FailureToParseFile)
        }
    }
}
