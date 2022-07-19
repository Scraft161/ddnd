/* Main program file
 * This is where we initialize our program and kick off into all the scripts we need
*/

mod story;
mod actions;

fn main() {
	// Story directory, what is the name of the directory we should look in for story files.
	let story_dir = "story/";
	// Initial file to load, this should be the file path relative to the "story/" directory and without extension
	let initial_file = "main";
	// Extension type, simple string with the character sequence after the "."
	let extension_type = ".ddnd";

	// get the path to the initial file.
	let initial_file_path = story_dir.to_owned() + initial_file + extension_type;

	// Start running the story file
	story::parse(&initial_file_path, true);
}
