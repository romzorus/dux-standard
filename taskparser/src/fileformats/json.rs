use config::Config;
use taskexec::workflow::task::TaskList;

pub fn json_tasklist_parser(raw_content: Config) -> TaskList {
    raw_content
        .try_deserialize::<TaskList>()
        .expect("Problem parsing the content of the file")
}