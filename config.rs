use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct Config {
    data_source: String,
    indexing_settings: HashMap<String, String>,
    environment_variables: HashMap<String, String>,
}

impl Config {
    fn load_from_file(file_path: &str) -> Result<Config, io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }

    fn load_env_variables(&mut self) {
        for (key, _) in &self.environment_variables {
            if let Ok(val) = env::var(key) {
                self.environment_variables.insert(key.clone(), val);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config.json";
    let mut config = Config::load_from_file(config_path)?;
    config.load_env_variables();
    println!("{:?}", config);

    Ok(())
}