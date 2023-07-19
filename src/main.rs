use rand::seq::SliceRandom;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value_t = 1)]
	amount: usize,
	#[arg(short, long, default_value_t = String::from(" "))]
	joiner: String
}

fn main() {
	let args = Args::parse();
	let bytes = include_bytes!("words.txt");
	let words: Vec<String> = bytes
		.lines()
		.collect::<Result<_, _>>()
		.expect("file parsed correctly");
	let words = get_random_words(&words, args.amount);
	println!("{}", words.join(&args.joiner));
}

fn get_random_words(words: &[String], amount: usize) -> Vec<String> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount)
		.cloned()
		.collect()
}
