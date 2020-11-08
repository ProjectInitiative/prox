#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate dirs;
extern crate rpassword;

mod options;
mod config;
mod defaults;

use rpassword::read_password;

extern crate rand;
use rand::Rng; 
use rand::distributions::Alphanumeric;

use std::{thread, time};
use std::sync::{Arc, Mutex};

use std::io::Read;
use rocket::{response::content, State};
use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status, ContentType};

use serde::{Serialize, Deserialize};

static SOME_INT: i32 = 5;

static BASE_URL: &str = "/api/v1";
const LIMIT: u64 = 256;

#[derive(Serialize, Deserialize)]
struct VMInfoList
{
    vm_queue: Vec<VMInfo>,
}

#[derive(Clone, Serialize, Deserialize)]
struct VMInfo
{
    title: String,
    description: String,
    id: i32,
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
        Success(serde_json::from_str(&string).unwrap())
    }
}

#[get("/")]
fn index() -> String
{
    format!("Hello!\nOne day I will add a nice UI or instructions to setup and view via a web interface!")
}

#[get("/vms-in-queue")]
fn vms_in_queue(queue: State<Arc<Mutex<Vec<VMInfo>>>>) -> content::Json<String>
{   
    let vm_info = VMInfoList { vm_queue: queue.lock().unwrap().clone() };
    content::Json(serde_json::to_string(&vm_info).unwrap())
}


#[post("/add-vm-to-queue", format = "application/json", data = "<vm_to_add>")]
fn add_vm_to_queue(queue: State<Arc<Mutex<Vec<VMInfo>>>>, vm_to_add: VMInfo) -> content::Json<String>
{
    // println!("hello");
    queue.lock().unwrap().push(vm_to_add);
    content::Json("".to_string())
}

#[delete("/delete-vm-from-queue", format = "application/json", data = "<vm_to_delete>")]
fn delete_vm_from_queue(vm_to_delete: VMInfo) -> content::Json<String>
{

    content::Json("".to_string())
}

fn main() {
    let _matches = options::get_matches();
    let app_config = config::AppConfig::new("prox".to_string(), ".toml".to_string(), defaults::default_values());
    
    println!("{:?}",app_config.read_config());

    let vm_queue = Arc::new(Mutex::new(Vec::<VMInfo>::new()));

    // println!("Type a password: ");
    // let password = read_password().unwrap();
    // println!("The password: {}", password);
    let key = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .collect::<String>();
    println!("{}",key);

    thread::spawn(move || {
        // some work here
    });
    
    rocket::ignite()
        .manage(vm_queue)
        .mount("/", routes![index])
        .mount(BASE_URL, routes![vms_in_queue, add_vm_to_queue])
        .launch();
    
    
    return;
}