use clap::Parser;
use crate::Case;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
	/// The string that joines the random words, if there are more than 1
	#[arg(short, long, default_value_t = String::from(" "))]
	joiner: String,
	/// Uppercase every word
	#[arg(short, long)]
	pub caps: bool,
	/// Titlecase every word
	#[arg(short, long)]
	pub title: bool,
	/// Disable interpreting \n as a newline and \t as a tab
	#[arg(short, long, default_value_t = false)]
	raw: bool,
	/// Amount of random words to print
	#[arg(default_value_t = 1)]
	pub amount: usize,
}

impl Args {
	pub fn get_case(&self) -> Case {
		if self.caps {
			Case::Caps
		} else if self.title {
			Case::Title
		} else {
			Case::Lower
		}
	}

	pub fn joiner(&self) -> String {
		match self.raw {
			false => self.joiner
				.clone()
				.replace("\\n", "\n")
				.replace("\\t", "\t"),
			true => self.joiner.clone(),
		}
	}
}