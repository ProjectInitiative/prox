#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate dirs;
extern crate rpassword;

mod options;
mod config;
mod defaults;
mod vminfo;
mod apiv1;

use crate::apiv1::*;
use crate::vminfo::VMInfo;
use std::time::Duration;
use rpassword::read_password;

extern crate rand;
use rand::Rng; 
use rand::distributions::Alphanumeric;

use std::{thread, time};
use std::sync::{Arc, Mutex};

use std::process::Command;

use rocket::{response::content, State};


static POLL_RATE: Duration = Duration::from_millis(1000);


fn poll_vm_queue(queue: Arc<Mutex<Vec<VMInfo>>>)
{
    loop
    {
        if !queue.lock().unwrap().is_empty()
        {

            let output = Command::new("sh")
                                .arg("-c")
                                .arg(format!("qm status {0}", queue.lock().unwrap()[0].id))
                                .output()
                                .expect("failed to execute process");
    
            let qemu_status = output.stdout;
            println!("{}",std::str::from_utf8(&qemu_status).unwrap());
            if std::str::from_utf8(&qemu_status).unwrap().contains("stopped")
            {
                if queue.lock().unwrap().len() >= 2
                {
                    //attempt to start the next VM in the queue
                    let output = Command::new("sh")
                                    .arg("-c")
                                    .arg(format!("qm start {0}", queue.lock().unwrap()[1].id))
                                    .output()
                                    .expect("failed to execute process");
                    
                    let qemu_status = output.stdout;
                    println!("{}",std::str::from_utf8(&qemu_status).unwrap());
                    if std::str::from_utf8(&qemu_status).unwrap().contains("running")
                    {
                        //remove old VM from the list
                        queue.lock().unwrap().pop();
                    }
                }
            }
        }
        thread::sleep(POLL_RATE);
    }
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

    let poll_queue = vm_queue.clone();
    thread::spawn(move || {
        // some work here
        poll_vm_queue(poll_queue);
    });
    
    rocket::ignite()
        .manage(vm_queue)
        .mount("/", routes![index])
        .mount(BASE_URL_V1, routes![vms_in_queue, add_vm_to_queue])
        .launch();
    
    
    return;
}