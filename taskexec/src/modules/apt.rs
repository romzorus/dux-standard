// APT Module : handle packages in Debian-like distributions

use serde::Deserialize;
use crate::workflow::{change::ModuleBlockChange, result::{ModuleBlockResult, TaskResult}};
use crate::modules::ModuleBlock;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AptBlock {
    action: String,
    package: Option<String>,
}

impl AptBlock {
    pub fn dry_run_block(&self) -> ModuleBlockChange {
        match self.action.as_str() {
            "install" => {
                ModuleBlockChange {
                    module: Some(
                        ModuleBlock::Apt(AptBlock{
                            action: "install".to_string(),
                            package: Some(self.package.clone().unwrap())
                        })
                    )
                }
            }
            "remove" => { ModuleBlockChange::new_none() }
            _ => { ModuleBlockChange::new_none() }
        }
    }

    pub fn apply_moduleblock_change(&self) -> ModuleBlockResult {
        match self.action.as_str() {
            "install" => {
                println!("**** Install package {}", self.package.clone().unwrap());
                ModuleBlockResult::from(
                    Some(0),
                    Some(format!("Installation de {} réussie !", self.package.clone().unwrap())),
                    None)

            }
            "remove" => {
                println!("**** Remove package {}", self.package.clone().unwrap());
                ModuleBlockResult::new_none()
            }
            _ => { ModuleBlockResult::new_none() }
        }
    }
}