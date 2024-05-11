use sha2::{Sha256, Digest};
use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use std::time::SystemTime;

use errors::Error;

#[derive(Debug)]
pub struct CorrelationIdGenerator {
    salt: String,
    value: String
}

impl CorrelationIdGenerator {

    pub fn new() -> CorrelationIdGenerator {
        CorrelationIdGenerator {
            salt: String::new(),
            value: String::new()
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let saltbuilding = IdBuilder::new(Encryption::MD5)
            .add_component(HWIDComponent::CPUID)
            .add_component(HWIDComponent::MacAddress)
            .add_component(HWIDComponent::MachineName)
            .add_component(HWIDComponent::Username)
            .build("dux");

        match saltbuilding {
            Ok(salt) => {
                self.salt = salt;
                return Ok(());
            }
            Err(e) => {
                return Err(Error::FailedInitialization(format!("{}", e)));
            }
        }

        
    }

    pub fn get_new_value(&mut self) -> Result<String, Error> {
        if self.salt.is_empty() {
            return Err(Error::MissingInitialization);
        } else {
            let now = SystemTime::now();
            let value = Sha256::digest(format!("{}{:?}", self.salt, now));
            self.value =  format!("{:X}", value);
            return Ok(self.value.clone())
        }
    }
}