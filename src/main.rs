use serde_json;
use serde_json::Value as JsonValue;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

fn main() {
	let file = get_json_path();
	let contents = read(file).unwrap();
	println!("{:#?}", contents)
}

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

fn read<P: AsRef<Path>>(path: P) -> Result<JsonValue, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let json = serde_json::from_reader(reader)?;
	Ok(json)
}
