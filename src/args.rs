use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Single-end FASTQ file (required)
    #[arg(short = '1', long = "one", required = true)]
    pub one: String,

    /// Paired-end FASTQ file (optional)
    #[arg(short = '2', long = "two")]
    pub two: Option<String>,

    /// Number of times to greet (example argument)
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

