use rand::Rng;
use serde_json::Value as JsonValue;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

fn main() {
	let file = get_json_path().unwrap();
	let json = read(file).unwrap();
	let amount = get_amount();
	for _ in 0..amount {
		println!("{}", get_random_word(&json));
	}
}

fn get_json_path() -> Result<PathBuf, io::Error> {
	let curr_exe = env::current_exe()?;
	let project_dir = curr_exe.parent()
		.expect("release / debug")
		.parent()
		.expect("target")
		.parent()
		.expect("uclanr");
	let json_file = project_dir.join("data")
		.join("words.json");
	Ok(json_file)
}

fn read<P: AsRef<Path>>(path: P) -> Result<JsonValue, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let json = serde_json::from_reader(reader)?;
	Ok(json)
}

fn get_random_word(json: &JsonValue) -> String {
	let mut rng = rand::thread_rng();
	let random_number = rng.gen_range(0..json.as_array().unwrap().len());
	json[random_number].as_str().unwrap().to_string()
}

fn get_amount() -> u32 {
	let mut args = env::args();
	if args.next().is_none() {
		// if the first argument isn't the executable, somehow
		return 1;
	};
	let amount = match args.next() {
		// if the second (meaning first) argument wasn't passed by the user
		Some(v) => v,
		None => return 1,
	};
	let amount: u32 = match amount.trim().parse() {
		// if it's not a proper number (the user is supposed to enter a number of random words to print)
		Ok(v) => v,
		Err(_) => return 1,
	};
	amount
}
