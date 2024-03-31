pub mod blocks;
pub mod apt;
pub mod yumdnf;

use serde::Deserialize;
use crate::workflow::change::ModuleBlockChange;
use crate::modules::blocks::*;
use connection::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum ModuleBlock {
    None, // Used for new() methods, initializations and errors
    Apt(AptBlock),
    Dnf(YumDnfBlock),
    Yum(YumDnfBlock)
}

impl ModuleBlock {
    pub fn new() -> ModuleBlock { ModuleBlock::None }

    pub fn dry_run_moduleblock(&self, hosthandler: &mut HostHandler) -> ModuleBlockChange {
        match &self {
            ModuleBlock::None => { ModuleBlockChange::none() }
            ModuleBlock::Apt(block) => { block.dry_run_block(hosthandler) }
            ModuleBlock::Dnf(block) => { block.dry_run_block(hosthandler) }
            ModuleBlock::Yum(block) => { block.dry_run_block(hosthandler) }
        }
    }
}