use std::process::exit;
use config::FileFormat;
use taskexec::workflow::task::TaskList;
use crate::fileformats::json::json_tasklist_parser;
use crate::fileformats::toml::toml_tasklist_parser;
use crate::fileformats::yaml::yaml_tasklist_parser;

pub fn tasklist_parser(tasklistcontent: String, format: FileFormat) -> TaskList {

    match format {
        FileFormat::Json => { json_tasklist_parser(&tasklistcontent) }
        FileFormat::Toml => { toml_tasklist_parser(&tasklistcontent) }
        FileFormat::Yaml => { yaml_tasklist_parser(&tasklistcontent) }
        _ => { exit(1) } // Placeholder : error handling required here
    }
}

pub fn tasklist_get_from_file(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap() // Placeholder : error handling required here
}