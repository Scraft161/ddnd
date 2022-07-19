/* Main program file
 * This is where we initialize our program and kick off into all the scripts we need
*/

mod story;
mod actions;
mod config;

fn main() {
	// Get config values from config.toml
	let global_config = config::get_config();

	let story_dir: String = global_config.story_dir.unwrap_or("story".to_owned());
	let initial_file: String = global_config.initial_file.unwrap_or("main".to_owned());
	let extension: String = global_config.extension.unwrap_or("ddnd".to_owned());

	// get the path to the initial file.
	let initial_file_path = format!("{}/{}.{}", story_dir, initial_file, extension);
	
	// Start running the story file
	story::parse(&initial_file_path, 0, true);
}
