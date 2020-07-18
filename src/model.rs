extern crate serde_yaml;
extern crate serde; 

use std::option::NoneError;

#[path = "model/config.rs"] pub mod config;
#[path = "model/driver.rs"] pub mod driver;
#[path = "model/user.rs"] pub mod user;
#[path = "model/object.rs"] pub mod object;
#[path = "model/token.rs"] pub mod token;
#[path = "model/redis.rs"] pub mod redis;

pub enum MyError {
    NoneError(NoneError)
}

impl From<NoneError> for MyError {
    fn from(none: NoneError) -> MyError {
        MyError::NoneError(none)
    }
}