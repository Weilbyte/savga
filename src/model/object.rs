extern crate serde; 

use serde::Deserialize;
use rocket::Outcome;
use rocket::State;
use rocket::Outcome::*;
use rocket::Data;
use std::fs::File;
use std::io::Read;
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition, MultipartFormDataError};
use crate::model;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Object {
    pub filename: String,
    pub raw: Vec<u8>
}

impl FromDataSimple for Object {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Object, ()> {
        let savga_config = request.guard::<State<model::config::Config>>().expect("Error reading Config state.");

        let options = MultipartFormDataOptions::with_multipart_form_data_fields(
            vec![
                match MultipartFormDataField::file("file")
                    .size_limit((savga_config.upload_size_limit as u64) * 1024 * 1024)
                    .content_type_by_string(Some(mime::IMAGE_STAR)) {
                        Ok(v) => v,
                        Err(_) => { return Failure((Status::raw(500), ())) }
                    }
            ]
        );
    
        let content_type = request.content_type().expect("Error getting Content-Type from request.");

        let form_data = match MultipartFormData::parse(&content_type, data, options) {
            Ok(v) => v,
            Err(e) => {
                    if e.to_string().contains("The boundary cannot be found.") {
                        return Failure((Status::raw(400), ()))
                    } else if e.to_string().contains("The data type of field `file` is incorrect.") {
                        return Failure((Status::raw(415), ()))
                    } else if e.to_string().contains("The data of field `file` is too large.") {
                        return Failure((Status::raw(413), ()))
                    } else {
                        return Failure((Status::raw(500), ()))
                    }
            }
        };
    
        let file = form_data.files.get("file");
    
        if let Some(fields) = file {
            let field = &fields[0];
            println!("{:?}", field);
            let filename = field.file_name.clone();

            let mut file = File::open(field.path.clone()).expect("Error reading tmp file.");
            let mut buffer = Vec::new();

            file.read_to_end(&mut buffer).expect("Error reading tmp file to vector.");

            return Success(Object {
                filename: filename.unwrap(),
                raw: buffer
            })
        }

        Failure((Status::raw(400), ()))
    }
}