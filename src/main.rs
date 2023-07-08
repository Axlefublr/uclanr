use rand::{rngs::ThreadRng, Rng};
use std::{
	env,
	error::Error,
	fs::File,
	io::{self, BufRead, BufReader},
	path::{Path, PathBuf},
};

fn main() {
	let file = get_json_path().unwrap();
	let json = read(file).unwrap();
	let amount = get_amount();
	let mut rng = rand::thread_rng();

	for _ in 0..amount {
		println!("{}", get_random_word(&json, &mut rng));
	}
}

fn get_json_path() -> Result<PathBuf, io::Error> {
	let curr_exe = env::current_exe()?;
	let project_dir = curr_exe
		.parent()
		.expect("release / debug")
		.parent()
		.expect("target")
		.parent()
		.expect("uclanr");
	let json_file = project_dir.join("data").join("words.txt");
	Ok(json_file)
}

fn read<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let json: Result<Vec<String>, io::Error> = reader.lines().collect();
	Ok(json?)
}

fn get_random_word(json: &[String], rng: &mut ThreadRng) -> String {
	let random_number = rng.gen_range(0..json.len());
	json[random_number].clone()
}

fn get_amount() -> u32 {
	let mut args = env::args();
	if args.next().is_none() {
		// if the first argument isn't the executable, somehow
		return 1;
	};

	args.next()
		.map(|arg| arg.trim().parse::<u32>().unwrap_or(1))
		.unwrap_or(1)
}
