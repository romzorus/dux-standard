use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::modules::apt::AptBlock;

pub mod apt;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ModuleBlock {
    None, // Used for new() methods, initializations and errors
    Apt(AptBlock)
}

impl ModuleBlock {
    pub fn new() -> ModuleBlock { ModuleBlock::None }

    pub fn dry_run_moduleblock(&self, host: String) -> ModuleBlockChange {
        match &self {
            ModuleBlock::None => { ModuleBlockChange::new_none() }
            ModuleBlock::Apt(block) => { block.dry_run_block(host) }
        }
    }
}