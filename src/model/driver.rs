extern crate serde; 
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Driver {
    pub kind: String,
    pub api: Option<String>
}

impl Default for Driver {
    fn default() -> Driver {
        Driver {
            kind: "local".to_string(),
            api: None
        }
    }
}