use std::process::exit;
use taskexec::workflow::task::TaskList;
use crate::fileformats::ContentFormat;
use crate::fileformats::json::json_tasklist_parser;
use crate::fileformats::toml::toml_tasklist_parser;
use crate::fileformats::yaml::yaml_tasklist_parser;

pub fn tasklist_parser(tasklistcontent: String, format: ContentFormat) -> TaskList {

    match format {
        ContentFormat::Json => { json_tasklist_parser(&tasklistcontent) }
        ContentFormat::Toml => { toml_tasklist_parser(&tasklistcontent) }
        ContentFormat::Yaml => { yaml_tasklist_parser(&tasklistcontent) }
        _ => { exit(1) } // Placeholder : error handling required here
    }
}

pub fn tasklist_get_from_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap() // Placeholder : error handling required here
}

pub fn tasklist_get_from_interactive_mode() -> String {
    // Placeholder : interactive_mode is when the final user sets a group of hosts and can
    // type commands as in a normal shell interpreter but each command is directly turned
    // into a Task, executed on the group of hosts, and the results arrive in "realtime".
    String::new()
}