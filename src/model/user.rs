extern crate serde; 
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct User {
    pub username: String, 
    pub password: String, 
    pub token: String
}