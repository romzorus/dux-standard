// This part is used to generate a ChangeList based on an Assignment.

use crate::workflow::result::TaskListResult;
use crate::modules::Module;
use crate::workflow::run::applychange;

pub struct ChangeList {
    pub list: Vec<Change>,
}

#[derive(Clone)]
pub struct Change {
    pub module: Module,
    pub action: String,
    pub parameters: Vec<String>
}

impl ChangeList {
    pub fn new() -> ChangeList {
        ChangeList {
            list: Vec::<Change>::new(),
        }
    }

    pub fn apply(&self) -> TaskListResult {

        let mut tasklistresult = TaskListResult::new();
        for change in self.list.iter() {
            let taskresult = applychange(change.clone());
            tasklistresult.results.push(taskresult);
        }
        tasklistresult
    }
}
