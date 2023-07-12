use rand::seq::SliceRandom;
use std::env;
use std::io::BufRead;

fn main() {
	let amount = get_amount();
	let bytes = include_bytes!("words.txt");
	let words: Vec<String> = bytes
		.lines()
		.collect::<Result<_, _>>()
		.expect("file parsed correctly");
	let words = get_random_words(&words, amount);
	println!("{}", words.join(" "));
}

fn get_random_words(words: &[String], amount: usize) -> Vec<String> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount)
		.cloned()
		.collect()
}

fn get_amount() -> usize {
	let Some(amount) = env::args().nth(1) else { return 1 };
	amount.trim().parse().unwrap_or(1)
}
