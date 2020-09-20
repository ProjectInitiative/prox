extern crate dirs;
extern crate rpassword;

mod options;
mod config;
mod defaults;

use rpassword::read_password;

extern crate rand;
use rand::Rng; 
use rand::distributions::Alphanumeric;

fn main() {
    let _matches = options::get_matches();
    let app_config = config::AppConfig::new("prox".to_string(), ".toml".to_string(), defaults::default_values());
    
    println!("{:?}",app_config.read_config());


    // println!("Type a password: ");
    // let password = read_password().unwrap();
    // println!("The password: {}", password);

    let key = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(20)
    .collect::<String>();
    println!("{}",key);

    return;
}