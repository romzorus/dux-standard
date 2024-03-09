// This part is used to generate a ChangeList based on an Assignment.

use crate::result::ExecResult;

pub struct ChangeList {
    changelist: Vec<Change>,
}

pub struct Change {
    change: String,
}

impl ChangeList {
    pub fn new() -> ChangeList {
        ChangeList {
            changelist: Vec::<Change>::new(),
        }
    }

    pub fn run(&self) -> ExecResult {
        ExecResult {
            exitcode: 0,
            stdout: String::from("stdout"),
            stderr: String::from("stderr")
        }
    }
}
