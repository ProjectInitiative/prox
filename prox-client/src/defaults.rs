#![allow(unused)]

use std::collections::HashMap;

pub fn default_values() -> HashMap<String, String>
{
    let mut values: HashMap<String, String> = HashMap::new();
    values.insert("server_id".to_string(),"prox-server".to_string());
    // values.insert("debug".to_string(),"false".to_string());
    // values.insert("cache".to_string(),"true".to_string());
    
    values
}