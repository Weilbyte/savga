extern crate serde; 
use serde::Deserialize;
use std::fs;

use crate::model;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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

impl Driver {
    pub fn upload(& self, data_dir: String, username: String, object: model::object::Object) {
        if self.kind == "gcp" {
            println!("Uploading to GCP: {:?}", object.raw);
        } else {
            println!("Saving locally: {:?}", object.raw);
        }
    }

    fn upload_local(data_dir: String, username: String, object: model::object::Object) -> String {
        let directory_path = format!("{}/{}/", data_dir, username);

        match fs::create_dir_all(directory_path) {
            Ok(_) => (),
            Err(e) => { eprintln!("Error creating user data directory: {}", e); return String::new() }
        };

        "noop".to_string()
    }
}