mod args;
mod struct_read_name;
mod functions;
use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();
    println!("Single-end file: {}", args.one);

    if let Some(pe_file) = args.two {
        println!("Paired-end file: {}", pe_file);
    } else {
        println!("Running in single-end mode.");
    }
}