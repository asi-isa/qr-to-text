use clap::{arg, Parser};

#[derive(Parser)]
pub struct Args {
    /// Path to images
    #[arg(short, long)]
    pub path: String,
}

impl Args {
    pub fn get_args() -> Self {
        Args::parse()
    }
}
