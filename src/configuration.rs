use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Deserialize)]
pub struct Configuration {
    pub dictionaries: HashMap<String, HashMap<String, String>>,
    pub keys: HashMap<String, String>,
    pub direct: HashMap<String, String>,
    pub layout: String,
}

pub type DictEntry<'a> = (&'a String, &'a String);
pub type HardDictEntry = (String, String);

pub trait DisplayableTypeable: Sized {
    fn display(&self) -> String;
    fn matchable(&self) -> String;
}

impl<'a> DisplayableTypeable for DictEntry<'a> {
    fn display(&self) -> String {
        self.0.clone()
    }

    fn matchable(&self) -> String {
        if self.1 == "" {
            self.0.clone()
        } else {
            self.1.clone()
        }
    }
}

impl DisplayableTypeable for HardDictEntry {
    fn display(&self) -> String {
        self.0.clone()
    }

    fn matchable(&self) -> String {
        // For some reason loading an empty value for a dict entry is '~'.
        if self.1 == "~" {
            self.0.clone()
        } else {
            self.1.clone()
        }
    }
}
pub trait HardCopyable {
    fn hard_copy(&self) -> (String, String);
}

impl<'a> HardCopyable for DictEntry<'a> {
    fn hard_copy(&self) -> HardDictEntry {
        (self.0.clone(), self.1.clone())
    }
}

impl Configuration {
    pub fn new(file_name: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
        let file = File::open(file_name)?;
        let conf: Configuration = serde_yaml::from_reader(file)?;
        Ok(conf)
    }
}
