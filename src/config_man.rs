extern crate yaml_rust;
extern crate serde_yaml;

use crate::model;

use std::env;
use std::path::Path;
use std::process::exit;
use std::fs;

pub fn run_checks() {
    let config_path = match env::var("SAVGA_CONF_PATH") {
        Ok(v) => v,
        Err(_) => { eprintln!("Error: Environment variable SAVGA_CONF_PATH is not provided."); exit(1); }
    };

    if !Path::new(&config_path).exists() {
        eprintln!("Error: Provided config file path is not valid.");
        exit(1);
    };
}

pub fn get_config() -> model::config::Config {
    let config_path = env::var("SAVGA_CONF_PATH").expect("Error while reading SAVGA_CONF_PATH env var");
    let config_string = match fs::read_to_string(Path::new(&config_path)) {
        Ok(v) => v,
        Err(e) => { eprintln!("Error while reading config file to memory: {}", e); exit(1); }
    };
 
    let config_object : model::config::Config = serde_yaml::from_str(&config_string).expect("Could not load YAML file from string");

    config_object
}

pub fn ensure_data_dir(path: &String) {
    match fs::create_dir_all(path) {
        Ok(_) => (),
        Err(e) => { eprintln!("Error while creating data directory: {}", e); exit(1); }
    }
}