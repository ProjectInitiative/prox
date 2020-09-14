extern crate dirs;

mod options;
mod config;
mod defaults;

fn main() {
    let _matches = options::get_matches();
    let app_config = config::AppConfig::new("prox".to_string(), ".toml".to_string(), defaults::default_values());
    
    println!("{:?}",app_config.read_config());

    return;
}