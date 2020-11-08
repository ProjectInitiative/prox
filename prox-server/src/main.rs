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
use rocket::{response::content, State};

use serde::{Serialize, Deserialize};

static SOME_INT: i32 = 5;

static BASE_URL: &str = "/api/v1";

#[derive(Serialize, Deserialize)]
struct VMInfo
{
    vm_queue: Vec<i32>,
}

#[get("/")]
fn index() -> String
{
    format!("Hello!\nOne day I will add a nice UI or instructions to setup and view via a web interface!")
}

#[get("/vms-in-queue")]
fn vms_in_queue(queue: State<Arc<Mutex<Vec<i32>>>>) -> content::Json<String>
{   
    let vm_info = VMInfo { vm_queue: queue.lock().unwrap().clone() };
    let serialized_vm_info = 
    queue.lock().unwrap().push(10);
    // let json = format!("{{ \"hi\": \"{0}\" }}",queue.lock().unwrap().last().unwrap().to_string());
    // let x = &json[..];
    let y = format!("{{ \"hi\": \"{0}\" }}",queue.lock().unwrap().last().unwrap()).as_str();
    // content::Json(format!("{{ \"hi\": \"{0}\" }}",queue.lock().unwrap().last().unwrap()))
    // content::Json(y)
    content::Json(format!("{{ \"hi\": \"{0}\" }}",queue.lock().unwrap().last().unwrap()))
}


#[post("/add-vm-to-queue")]
fn add_vm_to_queue()
{

}

fn main() {
    let _matches = options::get_matches();
    let app_config = config::AppConfig::new("prox".to_string(), ".toml".to_string(), defaults::default_values());
    
    println!("{:?}",app_config.read_config());

    let vm_queue = Arc::new(Mutex::new(Vec::<i32>::new()));

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