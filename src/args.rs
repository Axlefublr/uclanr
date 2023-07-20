use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
	#[arg(short, long, default_value_t = String::from(" "))]
	pub joiner: String,
	pub amount: Option<usize>,
}
