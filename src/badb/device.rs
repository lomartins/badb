use std::fmt;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Device {
    pub serial: String,
    pub model: String,
    pub os_version: String,
    pub ip: String,
}

impl Device {
    pub fn new(serial: String, model: String, os_version: Option<String>, ip: Option<String>) -> Device {
        Device {
            serial,
            model,
            os_version: os_version.unwrap_or("Undefined".to_string()),
            ip: ip.unwrap_or("Undefined".to_string()),
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0: <15}\tModel: {1: <5} OS: {2: <5}", self.serial.clone(), self.model.clone(), self.os_version.clone())
    }
}