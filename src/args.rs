use clap::Parser;
use crate::Case;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
	#[arg(short, long, default_value_t = String::from(" "))]
	pub joiner: String,
	#[arg(short, long)]
	pub caps: bool,
	#[arg(short, long)]
	pub title: bool,
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
}