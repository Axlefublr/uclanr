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
	let mut picked_words = get_random_words(&words, args.amount.unwrap_or(1));
	alter_case(&mut picked_words, args.get_case());
	println!("{}", picked_words.join(&args.joiner));
}

fn get_random_words(words: &[String], amount: usize) -> Vec<String> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount)
		.cloned()
		.collect()
}

pub enum Case {
	Caps,
	Title,
	Lower
}

fn alter_case(words: &mut [String], case: Case) {
	match case {
		Case::Caps => {
			words
				.iter_mut()
				.for_each(|x| {
					x.make_ascii_uppercase();
				});
		}
		Case::Title => {
			words
				.iter_mut()
				.for_each(|x| {
					x[0..1].make_ascii_uppercase();
				});
		}
		Case::Lower => ()
	}
}