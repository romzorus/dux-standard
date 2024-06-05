use taskexec::workflow::task::{TaskBlock, TaskList};
use hostparser::Host;
use errors::Error;
use tera::{Context, Tera};
use serde_json;

pub fn json_tasklist_parser(tasklistcontent: &String, host: &Host) -> Result<TaskList, Error> {

    // Before turning TaskList content into Rust struct, let's parse the variables
    let mut tera = Tera::default();
    let context = match &host.vars {
        Some(host_vars_list) => {
            Context::from_serialize(host_vars_list).unwrap()
        }
        None => {
            Context::new()
        }
    };

    let tasklist_content_with_vars = tera.render_str(tasklistcontent.as_str(), &context).unwrap();

    match serde_json::from_str::<Vec<TaskBlock>>(tasklist_content_with_vars.as_str()) {
        Ok(parsed_content) => {
            return Ok(TaskList::from(parsed_content));
        }
        Err(e) => {
            return Err(Error::FailureToParseContent(format!("{:?}", e)))
        }
    }
}
