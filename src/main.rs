//! Main module that handles and runs the uclanr program

// ----- Imports -----
use clap::{command, Parser};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
/// Uclanr - A cli app that generate random words
struct UclanrCliArgs {
	/// The number of words that should be printed on the stdout
	#[arg(short, long, default_value_t = 5, value_parser = amount_in_range)]
	amount: u16,
	/// Print words that startswith the word or letter provided
	#[arg(short, long, default_value_t = String::new())]
	startswith: String,
	/// Print words that endswith the word or letter provided
	#[arg(short, long, default_value_t = String::new())]
	endswith: String,
	/// Length of the words printed out
	#[arg(short, long, default_value_t = 5, value_parser = word_length_in_range)]
	length: u8,
}

// --- constants ---
const AMOUNT_RANGE: RangeInclusive<usize> = 1..=100;
const WORD_LENGTH_RANGE: RangeInclusive<usize> = 5..=16;

/// A helper function which sets a condition to check whether the amount argument provided is in
/// the range between 1 to 100.
fn amount_in_range(s: &str) -> Result<u16, String> {
	let amount: usize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
	if AMOUNT_RANGE.contains(&amount) {
		Ok(amount as u16)
	} else {
		Err(format!(
			"Amount not in range {}-{}",
			AMOUNT_RANGE.start(),
			AMOUNT_RANGE.end()
		))
	}
}

/// A helper function which sets a condition to check whether the word lenght argument provided is
/// in the range between 5 to 16.
fn word_length_in_range(s: &str) -> Result<u8, String> {
	let length: usize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;
	if WORD_LENGTH_RANGE.contains(&length) {
		Ok(length as u8)
	} else {
		Err(format!(
			"Word length not in range {}-{}",
			AMOUNT_RANGE.start(),
			AMOUNT_RANGE.end()
		))
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let uclanr_cli_args: UclanrCliArgs = UclanrCliArgs::parse();
	let words = BufReader::new(File::open("words.txt")?);
	// filters words read from the file according to the options provided and displays it on the
	// terminal
	let filtered_words: Vec<String> = words
		.lines()
		.into_iter()
		.filter(|word| word.clone().as_ref().unwrap().len() == (uclanr_cli_args.length as usize))
		.filter(|word| {
			word.clone()
				.as_ref()
				.unwrap()
				.starts_with(&uclanr_cli_args.startswith)
		})
		.filter_map(|word| {
			word.as_ref()
				.unwrap()
				.ends_with(&uclanr_cli_args.endswith)
				.then_some(word.unwrap().clone())
		})
		.collect();
	let words = random_words(&filtered_words, uclanr_cli_args.amount);
	println!("{}", words.join("\n"));
	Ok(())
}

/// A function which reads and gathers random words
fn random_words(words: &[String], amount: u16) -> Vec<String> {
	words
		.choose_multiple(&mut rand::thread_rng(), amount as usize)
		.cloned()
		.collect()
}
