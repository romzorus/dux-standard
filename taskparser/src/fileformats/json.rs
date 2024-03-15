use config::Config;
use taskexec::workflow::task::TaskList;

pub fn json_tasklist_parser(tasklistcontent: &str) -> TaskList {
    // tasklistcontent
    //     .try_deserialize::<TaskList>()
    //     .expect("Problem parsing the content of the file")
    TaskList::new()
}