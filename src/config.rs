#![allow(unused)]

extern crate config;
extern crate dirs;

use std::path::PathBuf;
use std::io::Write;
use std::fs::OpenOptions;

use std::collections::HashMap;

pub struct AppConfig 
{
    app_name: String,
    config_extension: String,
    default_values: HashMap<String, String>
}

impl AppConfig
{
    pub fn new
    (
        app_name: String, 
        config_extension: String, 
        default_values: HashMap<String, String>
    ) -> AppConfig 
    {
        AppConfig 
        { 
            app_name: app_name,
            config_extension: config_extension,
            default_values: default_values
        }
    }
    
    pub fn new_no_defaults
    (
        app_name: String, 
        config_extension: String
    ) -> AppConfig 
    {
        AppConfig 
        { 
            app_name: app_name,
            config_extension: config_extension,
            default_values: HashMap::new()
        }
    }

    pub fn config_loc(&self) -> PathBuf
    {
        // TODO: add in error checking
        let sys_config_dir: PathBuf = dirs::config_dir().unwrap();
        let app_config_dir: PathBuf = [sys_config_dir.to_str().unwrap(), &self.app_name].iter().collect();
    
        app_config_dir
    }

    pub fn config_file_loc(&self) -> PathBuf
    {
        [
            &self.config_loc().to_str().unwrap(), 
            &format!("{0}{1}", &self.app_name, &self.config_extension).as_str()
        ].iter().collect()
    }

    fn open_read_config(&self) -> HashMap<String, String>
    {
        let mut settings = config::Config::default();
        settings
            // Add in `$CONFIG/prox/prox.toml`
            .merge(config::File::with_name(&self.config_file_loc().to_str().unwrap())).unwrap()
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .merge(config::Environment::with_prefix("APP")).unwrap();
    
        settings.try_into::<HashMap<String, String>>().unwrap()
    }

    pub fn read_config(&self) -> HashMap<String, String> {
        let app_config_dir: PathBuf = self.config_loc();
        let app_config_file: PathBuf = self.config_file_loc();

        if (!&app_config_file.exists())
        {
            println!("Config file not found! Creating default: {}", &app_config_file.to_str().unwrap());
            std::fs::create_dir_all(app_config_dir);
            std::fs::File::create(&app_config_file);
        }
        
        let mut read_attempt = self.open_read_config();
        
        if(read_attempt.is_empty())
        {
            //TODO: check all keys in the "defaults" hashmap to make sure the config contains
            // said keys, if not add all missing keys with defaults

            // populate new file with default values
            self.save(&self.default_values);
            return self.open_read_config();
        }
        read_attempt
        
    }
    
    pub fn save(&self, config: &HashMap<String, String>) {
    
        // sort hashmap into an alphabetically ordered vector
        let mut sorted_config: Vec<_> = config.into_iter().collect();
        sorted_config.sort_by(|x,y| x.0.cmp(&y.0));
        
        if let Ok(mut file) = OpenOptions::new().write(true).open(&self.config_file_loc()) {
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
}

