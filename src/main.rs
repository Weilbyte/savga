#![feature(try_trait, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::{Config, Environment};
use std::process::exit;
use std::thread;
use std::sync::RwLock;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

mod config_man;
mod routes;
mod model;

use crate::routes::{index, api_basic};

fn main() {
    config_man::run_checks();
    let savga_config : model::config::Config = config_man::get_config();
    config_man::ensure_data_dir(&savga_config.data_dir);

    let redis = model::redis::Redis::new(savga_config.redis_uri.to_string());
    redis.subscribe_recache();

    start_server(savga_config, redis);
}

fn start_server(savga_config: model::config::Config, redis: model::redis::Redis) {
    let rocket_config = match Config::build(Environment::Production).address(&savga_config.address).port(savga_config.port).log_level(rocket::config::LoggingLevel::Normal).finalize() {
        Ok(v) => v,
        Err(_) => { eprintln!("Error while creating Rocket configuration instance, double-check port and address."); exit(1); }
    };
    
    let routes_to_mount = routes![index::index, api_basic::upload];
    rocket::custom(rocket_config)
        .mount("/", routes_to_mount)
        .manage(savga_config)
        .manage(redis)
        .launch();
}
