use taskexec::workflow::task::{Task, TaskList};
use serde_json;

pub fn json_tasklist_parser(tasklistcontent: &String) -> TaskList {
    let tasklist: Vec<Task> = serde_json::from_str(tasklistcontent).unwrap();
    return TaskList::from(tasklist)
}
