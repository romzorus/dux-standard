use serde::Deserialize;

pub mod apt;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum Module {
    None, // Used for new() methods, initializations and errors
    Apt
}