extern crate serde; 

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Token(String);

