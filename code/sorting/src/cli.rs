use clap::{Args, Parser};

#[derive(Parser)]
#[command(
    author = "Andreas Krath - https://github.com/andreaskrath",
    version = "0.0.0",
    about = "A tool that visualizes sorting algorithms",
    arg_required_else_help(true)
)]
pub enum Cli {
    /// The insertion sort algorithm
    Insertion(SortArgs),
    /// The bingo sort algorithm
    Bingo(SortArgs),
}

impl Cli {
    pub fn get_args() -> Self {
        Cli::parse()
    }
}

#[derive(Args)]
pub struct SortArgs {
    /// The size of the generated gif, specified in pixels, in a 1:1 ratio.
    /// The size also determines the amount of elements in the generated gif; one element per horizontal pixel.
    #[clap(value_name = "gif size")]
    #[arg(short = 's', long = "size", default_value_t = 100)]
    gif_size: usize,
}
