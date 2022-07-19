/* The main actions file.
 * This file handles the majority of our scripting
*/

use lazy_static::lazy_static;
use regex::Regex;

use crate::story;

lazy_static! {
	static ref REGEX_KEY_VAL: Regex = Regex::new(r"^!(?P<action>[A-Za-z0-9]+): (?P<argument>[A-Za-z0-9 ]+)").unwrap();
}

// Main parsing part of our scripting engine
pub fn run_action(item: &str, file_path: &str) {
	// Remove the prefix and do some very barebones tokenization
	let action_pair = REGEX_KEY_VAL.captures(item).unwrap();

	// Use the action to jump to the proper function
	match &action_pair["action"] {
		"loadScenario" => load_file(&action_pair["argument"], true),
		"loadScenarioReturn" => load_file(&action_pair["argument"], false),
		_ => unknown_action(&action_pair["action"], file_path),
	}

	// Bunch of commented out testing code, this function is being repurposed into the main scripting parser.

	//println!("[DBG]: parsing scene metadata {}", item);

	// split $item into $key and $val where $key is the string between "^!" and ": " and $val is the string between ": " and "$"
	//let pair = REGEX_KEY_VAL.captures(item).unwrap();

	//println!("Key: {}\nValue: {}", &pair["key"], &pair["val"]);
}

fn load_file(argument: &str, noreturn: bool) {
	// Temp hack for file load testing
	let file_path = "story/".to_owned() + argument + ".ddnd";

	story::parse(&file_path, noreturn);
}

fn unknown_action(action: &str, file: &str) {
	println!("[ERR]: Unknown action: \"{}\" in file: \"{}\".", action, file)
}