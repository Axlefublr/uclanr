use rand::seq::SliceRandom;
use std::env;

mod words;

fn main() {
	let amount = get_amount();
	let words = get_random_words(amount);
	println!("{}", words.join(" "));
}

fn get_random_words(amount: usize) -> Vec<&'static str> {
	words::WORDS
		.choose_multiple(&mut rand::thread_rng(), amount)
		.copied()
		.collect()
}

fn get_amount() -> usize {
	let Some(amount) = env::args().nth(1) else { return 1 };
	amount.trim().parse().unwrap_or(1)
}
