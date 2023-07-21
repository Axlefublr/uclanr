use clap::Parser;
use crate::Case;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
	#[arg(short, long, default_value_t = String::from(" "))]
	joiner: String,
	#[arg(short, long)]
	pub caps: bool,
	#[arg(short, long)]
	pub title: bool,
	#[arg(short, long, default_value_t = false)]
	raw: bool,
	pub amount: Option<usize>,
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