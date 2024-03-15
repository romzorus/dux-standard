// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::TaskListResult;
use crate::modules::Module;
use crate::workflow::run::apply_change;

#[derive(Debug, Clone)]
pub struct Change {
    pub module: Module,
    pub action: Option<String>,
    pub params: Option<Vec<String>>
}

impl Change {
    pub fn new() -> Change {
        Change {
            module: Module::None,
            action: Some(String::from("")),
            params: None
        }
    }

    pub fn none() -> Change {
        Change {
            module: Module::None,
            action: None,
            params: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChangeList {
    pub correlationid: String,
    pub list: Vec<Change>,
}

impl ChangeList {
    pub fn new(correlationid: String) -> ChangeList {
        ChangeList {
            correlationid,
            list: Vec::<Change>::new(),
        }
    }

    pub fn apply(&self) -> TaskListResult {

        let mut tasklistresult = TaskListResult::new(self.correlationid.clone());

        for change in self.list.iter() {
            let taskresult = apply_change(change.clone());
            tasklistresult.results.push(taskresult);
        }
        tasklistresult
    }
}
