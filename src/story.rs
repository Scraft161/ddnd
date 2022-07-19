/* Parse the story file.
 * This script both loads and parses our story files, they are our main scripts and fire off various other events like character dialogues
*/

extern crate lazy_static;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::process::exit;
//use lazy_static::lazy_static;
//use regex::Regex;

use crate::actions;

// We try to put our regex here instead of some other loop to not force regex recompilation
/*lazy_static! {
	static ref REGEX_KEY_VAL: Regex = Regex::new(r"^!(?P<key>[A-Za-z0-9]+): (?P<val>[A-Za-z0-9 ]+)").unwrap();
}*/

// load the given storyfile and parse it.
pub fn parse(file_path: &str, noreturn: bool) {
	println!("[INFO]: Looking for file: {}", file_path);

	let file_content = fs::read_to_string(file_path)
		.expect("[ERR]: Something went wrong trying to read file");

	// Now we start interpreting the file
	let scenario_per_line = file_content.split("\n");

	new_scene();

	scenario_per_line.for_each(|item: &str|
		//println!("> {}", item),
		match item {
			"---" => new_scene(),
			_ if item.starts_with("!") => actions::run_action(item, file_path),
			_ if item.starts_with("#") => (),	// Lines starting with a "#" will be comments
			"" => println!("{}", item),
			_ => line_print(item),
		}
	);

	// This point should only be reached when we run out of file.
	// To prevent possible undocumented behavior we just exit here.
	if noreturn {
		println!("[DDND]: Reached end of story file, that's all folks");

		exit(0);
	}
}

pub fn new_scene() {
	// Initialize a new scene.
	write!(io::stdout(), "{}", ansi_escapes::ClearScreen).unwrap();
	
	//println!("[DBG]: Changed Scene.");
}

fn line_print(text: &str) {
	// Prepare for jank as we need a cross-platform pause
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	// Now we print with the cursor at the end and flush manually
	write!(stdout, "{}", text).unwrap();
	stdout.flush().unwrap();

	// Finally read a single byte and discard (we will process it later)
	let _ = stdin.read(&mut [0u8]).unwrap();
}