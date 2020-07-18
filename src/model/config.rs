extern crate serde; 
use serde::Deserialize;

use super::user;
use super::driver;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_expiry")]
    pub upload_default_exp: u16,

    #[serde(default = "default_limit")]
    pub upload_size_limit: u16,

    #[serde(default = "default_panel")]
    pub panel: bool,

    #[serde(default)]
    pub driver: driver::Driver,

    #[serde(default)]
    pub users: Vec<user::User>
}

fn default_port() -> u16 {
    80
}

fn default_expiry() -> u16 {
    0
}

fn default_limit() -> u16 {
    50
}

fn default_panel() -> bool { 
    true
}


