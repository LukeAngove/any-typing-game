use std::fs::File;
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Configuration {
    pub dictionaries: HashMap<String, Vec<String>>,
    pub keys: HashMap<String, String>,
    pub layout: String,
}

impl Configuration {
    pub fn new(file_name : &String) -> Result<Configuration, Box<dyn std::error::Error>> {
        let file = File::open(file_name)?;
        let conf : Configuration = serde_yaml::from_reader(file)?;
        Ok(conf)
    }
}