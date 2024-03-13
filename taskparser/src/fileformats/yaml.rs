use taskexec::workflow::task::Task;
use serde_yaml;

pub fn yaml_tasklist_parser(file_path: &str) -> Vec<Task> {
    let tasklistfile = std::fs::File::open(file_path).unwrap();
    let tasklist: Vec<Task> = serde_yaml::from_reader(tasklistfile).unwrap();
    return tasklist;
}
