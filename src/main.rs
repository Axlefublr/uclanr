use rand::seq::SliceRandom;
use std::env;
use std::io::BufRead;
use std::io;

fn main() {
	let amount = get_amount();
	let bytes = include_bytes!("words.txt");
	let words: Result<Vec<String>, io::Error> = bytes
		.lines()
		.collect();
	let words = words.expect("file parsed correctly");
	let words = get_random_words(words, amount);
	println!("{}", words.join(" "));
}

fn get_random_words(words: Vec<&str>, amount: usize) -> Vec<&str> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount)
		.copied()
		.collect()
}

fn get_amount() -> usize {
	let Some(amount) = env::args().nth(1) else { return 1 };
	amount.trim().parse().unwrap_or(1)
}
