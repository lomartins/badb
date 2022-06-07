use std::fmt;

pub struct Device {
    pub serial: String,
    pub model: Option<String>,
    pub os_version: Option<String>,
}

impl Device {
    pub fn new(serial: String, model: Option<String>, os_version: Option<String>) -> Device {
        Device {
            serial,
            model,
            os_version,
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let model = self.model.clone().unwrap_or("Undefined".to_string());
        let os_version = self.os_version.clone().unwrap_or("Undefined".to_string());
        write!(f, "{}\tModel: {} - OS: {}", self.serial, model, os_version)
    }
}