use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, about, next_line_help = true)]
pub struct Args {
    /// The string that joines the random words, if there are more than 1
    #[arg(short, long, default_value_t = String::from(" "))]
    joiner: String,

    /// Sets the case of every word
    #[arg(short, long, value_enum, default_value_t = Case::Lower)]
    case: Case,

    /// Disable interpreting \n as a newline and \t as a tab
    #[arg(short, long)]
    raw: bool,

    /// Amount of random words to print
    #[arg(default_value_t = 1)]
    pub amount: usize,
}

impl Args {
    pub fn get_joiner(&self) -> String {
        if self.raw {
            self.joiner.clone()
        } else {
            self.joiner.clone().replace("\\n", "\n").replace("\\t", "\t")
        }
    }

    pub fn alter_case(&self, mut words: Vec<String>) -> Vec<String> {
        self.case.alter_case(&mut words);
        words.to_vec()
    }
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Case {
    Caps,
    Title,
    Lower,
}

impl Case {
    fn alter_case(self, words: &mut [String]) {
        match self {
            Self::Caps => {
                for x in words.iter_mut() {
                    x.make_ascii_uppercase();
                }
            }

            Self::Title => {
                for x in words.iter_mut() {
                    x[0..1].make_ascii_uppercase();
                }
            }

            Self::Lower => {}
        }
    }
}
