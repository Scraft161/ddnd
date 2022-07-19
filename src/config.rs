/* Import settings from the config.toml file
*/
/*
use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
    story_dir: String,
    initial_file: String,
    extension_type: String,
}

// Get full config data
pub fn get_config() {
	let config_path = "config.toml";
	let config_string = fs::read_to_string(config_path).expect("Something went wrong");
	let config: Config = toml::from_str(r#"
	story_dir = 'story'
	initial_file = 'main'
	extension_type = 'ddnd'
	"#).unwrap();
}

// Get single value from config
pub fn get_value(key: &str) {}

*/