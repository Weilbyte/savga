extern crate serde; 
extern crate redis;
use serde::Deserialize;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use redis::Client;

use crate::model;
use std::process::exit;

#[derive(Debug)]
pub struct Redis {
    pub uri: String,
    pub client: Arc<Client>
}

impl Redis {
    pub fn new(uri: String) -> Redis {
        let client = match redis::Client::open(uri.to_string()) {
            Ok(v) => v,
            Err(_) => { eprintln!("Redis URI not valid."); exit(1); }
        };

        match client.get_connection() {
            Ok(_) => (),
            Err(_) => { eprintln!("Could not connect to Redis server."); exit(1); }
        };

        return Redis {
            uri: uri, 
            client: Arc::new(client)
        }
    }

    pub fn subscribe_recache(& self) -> thread::JoinHandle<()> {
        let client = Arc::clone(&self.client);
        thread::spawn(move || {
            let mut conn = client.get_connection().expect("Could not get Redis connection.");
            let mut conn_ps = conn.as_pubsub();
            conn_ps.subscribe("recache").expect("Error subscribing to recache channel.");

            loop {
                let msg = conn_ps.get_message().expect("Error getting channel message.");
                let payload : String = msg.get_payload().expect("Error getting message payload.");
                println!("{}", payload);
            }
        })
    }
}