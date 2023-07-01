use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {

fn get_json_path() -> PathBuf {
	env::current_exe()
		.unwrap()
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join("data")
		.join("words.json")
}

}
