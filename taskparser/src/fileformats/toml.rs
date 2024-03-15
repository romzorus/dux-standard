use taskexec::workflow::task::TaskList;

pub fn toml_tasklist_parser(tasklistcontent: &String) -> TaskList {
    // tasklistcontent
    //     .try_deserialize::<TaskList>()
    //     .expect("Problem parsing the content of the file")
    TaskList::new()
}