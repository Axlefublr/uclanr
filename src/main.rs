use rand::seq::SliceRandom;
use std::io::BufRead;
use clap::Parser;

mod args;

fn main() {
	let args = args::Args::parse();
	let bytes = include_bytes!("words.txt");
	let words: Vec<String> = bytes
		.lines()
		.collect::<Result<_, _>>()
		.expect("file parsed correctly");
	let words = get_random_words(&words, args.amount.unwrap_or(1));
	println!("{}", words.join(&args.joiner));
}

fn get_random_words(words: &[String], amount: usize) -> Vec<String> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount)
		.cloned()
		.collect()
}
