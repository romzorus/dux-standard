use std::process::exit;

use config::{Config, File, FileFormat};
use taskexec::workflow::task::TaskList;
use crate::fileformats::json::json_tasklist_parser;
use crate::fileformats::toml::toml_tasklist_parser;
use crate::fileformats::yaml::yaml_tasklist_parser;

pub fn tasklist_parser(file_path: &str, file_format: FileFormat) -> TaskList {

    let tasklist_raw_content = Config::builder()
        .add_source(File::new(file_path, file_format))
        .build()
        .expect("Problem opening the tasks list file");

    match file_format {
        FileFormat::Json => { json_tasklist_parser(tasklist_raw_content) }
        FileFormat::Toml => { toml_tasklist_parser(tasklist_raw_content) }
        FileFormat::Yaml => { yaml_tasklist_parser(tasklist_raw_content) }
        _ => { exit(1) } // Placeholder : error handling required here
    }
}
