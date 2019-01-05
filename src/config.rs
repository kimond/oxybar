use toml;
use serde_derive::Deserialize;
use std::fs;
use std::io::{Read};
use std::error::Error;
use std::collections::HashMap;
use crate::modules::ModuleType;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub height: u32,
    pub left_modules: Vec<String>,
    pub right_modules: Vec<String>,
    pub modules: HashMap<String, ModuleAttrs>
}

#[derive(Debug, Deserialize)]
pub struct ModuleAttrs {
    pub mod_type: ModuleType,
    pub prefix: Option<String>,
    pub suffix: Option<String>
}


pub fn parse_config(path: String) -> Result<Config, Box<Error>> {
    let mut toml_string = String::new();
    fs::File::open(&path)?.read_to_string(&mut toml_string)?;

    let config = toml::from_str(&toml_string)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_works_correctly() {
        let config = parse_config("./tests/config.toml".to_string()).unwrap();
        println!("{:#?}", config);
        assert_eq!(config.height, 22);
    }
}
