use clap::Parser;
use rand::seq::SliceRandom;
use words::WORDS;

mod args;
mod words;

fn main() {
	let args = args::Args::parse();
	let picked_words = args.alter_case(get_random_words(args.amount));

	println!("{}", picked_words.join(&args.get_joiner()));
}

fn get_random_words(amount: usize) -> Vec<String> {
	WORDS
		.choose_multiple(&mut rand::thread_rng(), amount)
		.map(|word| (*word).to_string())
		.collect()
}
