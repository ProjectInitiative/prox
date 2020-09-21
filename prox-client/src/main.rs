#![allow(unused)]

extern crate dirs;
extern crate rpassword;
extern crate keyring;

mod options;
mod config;
mod defaults;

use rpassword::read_password;

extern crate rand;
use rand::Rng; 
use rand::distributions::Alphanumeric;

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

// static USERNAME: &str = "PROX-CLIENT";
static APP_NAME: &str = "prox";
static EXTENSION: &str = ".toml";




fn main() {
    let matches = options::get_matches();

    let app_config = config::AppConfig::new(APP_NAME.to_string(), EXTENSION.to_string(), defaults::default_values());
    let app_settings = app_config.read_config();

    let server_id = app_settings.get("server_id").unwrap();

    if matches.args.contains_key("deletecache")
    {
        println!("Deleting cached data ...");
        delete_password(&server_id);
        return;
    }

    println!("{:?}", &app_settings);

    let key = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(20)
    .collect::<String>();
    println!("{}",key);

    println!("password: {}", get_password(&server_id));

    return;
}

fn get_password(username: &str) -> String
{
    let service = "rust-keyring";
    let keyring = keyring::Keyring::new(&service, &username);

    let password = match keyring.get_password()
    {
        Ok(passwd) => passwd, 
        Err(_e) => set_new_password(&username),
    };

    password
}

fn set_new_password(username: &str) -> String
{
    let service = "rust-keyring";
    let keyring = keyring::Keyring::new(&service, &username);

    println!("Type the password for {0} (input hidden): ", &username);
    let password = read_password().unwrap();

    keyring.set_password(&password);

    password
}

fn delete_password(username: &str)
{
    let service = "rust-keyring";
    let keyring = keyring::Keyring::new(&service, &username);

    keyring.delete_password();
}