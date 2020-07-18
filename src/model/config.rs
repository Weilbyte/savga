extern crate serde; 
use serde::Deserialize;

use super::user;
use super::driver;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_addr")]
    pub address: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_expiry")]
    pub upload_default_exp: u16,

    #[serde(default = "default_limit")]
    pub upload_size_limit: u16,

    #[serde(default = "default_panel")]
    pub panel: bool,

    #[serde(default = "default_data")]
    pub data_dir: String,

    #[serde(default = "default_redis")]
    pub redis_uri: String,

    #[serde(default)]
    pub driver: driver::Driver,

    #[serde(default)]
    pub users: Vec<user::User>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            address: default_addr(),
            port: default_port(),
            upload_default_exp: default_expiry(),
            upload_size_limit: default_limit(),
            panel: default_panel(),
            data_dir: default_data(),
            redis_uri: default_redis(),
            driver: driver::Driver { ..Default::default() },
            users: vec![]
        }
    }
}

fn default_addr() -> String {
    "127.0.0.1".to_string()
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
fn default_data() -> String {
    "/opt/savga/".to_string()
}

fn default_redis() -> String {
    "redis://127.0.0.1:6379".to_string()
}

