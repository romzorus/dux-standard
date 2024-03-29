use taskexec::workflow::task::{Task, TaskList};
use serde_yaml;

pub fn yaml_tasklist_parser(tasklistcontent: &String) -> TaskList {
    let tasklist: Vec<Task> = serde_yaml::from_str(tasklistcontent).unwrap();
    return TaskList::from(tasklist);
}
