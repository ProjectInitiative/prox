#![allow(unused)]

use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

use std::io::Read;
const LIMIT: u64 = 256;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VMInfoList
{
    pub vm_queue: Vec<VMInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VMInfo
{
    pub title: String,
    pub description: String,
    pub id: i32,
}

impl FromDataSimple for VMInfo
{
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let vm_info_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&vm_info_ct) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        
        // add error checking for the JSON deserialization
        let deserialization = serde_json::from_str(&string);
        match deserialization
        {
            Ok(json) => return Success(json),
            Err(e) => return Failure((Status::InternalServerError, format!("{:?}", e)))
        };
        
    }
}