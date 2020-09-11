extern crate config;

use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

use std::collections::HashMap;

pub fn read_config() -> HashMap<String, String> {
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name("Settings")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let settings_hashmap = settings.try_into::<HashMap<String, String>>().unwrap();
    
    save(&settings_hashmap);
    
    settings_hashmap
}

pub fn save(config: &HashMap<String, String>) {
    let path = Path::new("test.toml");

    // sort hashmap into an alphabetically ordered vector
    let mut sorted_config: Vec<_> = config.into_iter().collect();
    sorted_config.sort_by(|x,y| x.0.cmp(&y.0));
    
    if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
        file.set_len(0).unwrap();
        for (key, value) in sorted_config {
            //TODO: find library, or manually add nested HashMap support along with list support
            file.write_all(
                format!(
                    "{0}=\"{1}\"\n",
                    key,
                    value
                ).as_bytes()
            ).unwrap();
        }
    }
}