use rand::seq::SliceRandom;
use std::env;

fn main() {
	let amount = get_amount();
	let words: Vec<&str> = include_str!("words.txt")
		.trim()
		.split('\n')
		.collect();
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
