#![allow(unused)]

use std::sync::{Arc, Mutex};
use crate::vminfo::VMInfo;
use crate::vminfo::VMInfoList;
use rocket::{response::content, State};

pub static BASE_URL_V1: &str = "/api/v1";


#[get("/")]
pub fn index() -> String
{
    format!("Hello!\nOne day I will add a nice UI or instructions to setup and view via a web interface!")
}

#[get("/vms-in-queue")]
pub fn vms_in_queue(queue: State<Arc<Mutex<Vec<VMInfo>>>>) -> content::Json<String>
{   
    let vm_info = VMInfoList { vm_queue: queue.lock().unwrap().clone() };
    content::Json(serde_json::to_string(&vm_info).unwrap())
}


#[post("/add-vm-to-queue", format = "application/json", data = "<vm_to_add>")]
pub fn add_vm_to_queue(queue: State<Arc<Mutex<Vec<VMInfo>>>>, vm_to_add: VMInfo) -> content::Json<String>
{
    // println!("hello");
    queue.lock().unwrap().push(vm_to_add);
    content::Json("".to_string())
}

#[delete("/delete-vm-from-queue", format = "application/json", data = "<vm_to_delete>")]
pub fn delete_vm_from_queue(vm_to_delete: VMInfo) -> content::Json<String>
{

    content::Json("".to_string())
}