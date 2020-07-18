extern crate serde; 

use serde::Deserialize;
use rocket::Outcome;
use rocket::State;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use crate::model;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct User {
    pub username: String, 
    pub password: String, 
    pub token: String
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let token = request.headers().get_one("x-api-key").unwrap_or("");
        let savga_config = request.guard::<State<model::config::Config>>()?;

        for user in savga_config.users.iter() {
            if token == user.token {
                return Outcome::Success(User {
                    username: user.username.to_string(),
                    password: user.password.to_string(),
                    token: user.token.to_string()
                })
            }
        }

        if token == "" {
            return Outcome::Failure((Status::BadRequest, ()))
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}