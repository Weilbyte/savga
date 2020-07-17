use std::env;
use std::path::Path;
use std::process::exit;

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