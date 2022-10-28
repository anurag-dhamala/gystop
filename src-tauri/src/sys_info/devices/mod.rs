use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    name: String,
}

impl Device {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}
