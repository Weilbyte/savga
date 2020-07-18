extern crate rocket_multipart_form_data;

use rocket;
use rocket::State;
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition};

use crate::model;

#[post("/object", format="multipart/form-data", data="<object>")]
pub fn upload(user: model::user::User, object: model::object::Object, savga_config: State<model::config::Config>) -> String {
    
    //savga_config.driver.upload();
    format!("Hello {}! Your uploaded file ({}) has been uploaded!", user.username, object.filename)

}