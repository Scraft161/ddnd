/* Import settings from the config.toml file
*/

use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub story_dir: Option<String>,
    pub initial_file: Option<String>,
    pub extension: Option<String>,
}

// Get full config data
pub fn get_config() -> Config {
	let config_path = "config.toml";
	let config_string = fs::read_to_string(config_path).expect("Could not read config file");
	let config: Config = toml::from_str(&config_string).unwrap();

	config
}

// Get single value from config
//pub fn get_value(key: &str) {}