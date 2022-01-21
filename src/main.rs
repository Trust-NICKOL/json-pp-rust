use clap::Parser;
use json_pp_rust::{process_json, Arguments};
use std::io;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Compact json string instead of pretty printing.
    #[clap(long)]
    compact: bool,
}

fn main() -> anyhow::Result<()> {
    // Arguments parsing
    let args = Args::parse();

    // Read from stdin
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;

    let print = process_json(Arguments::new(args.compact), buffer)?;

    // print result
    println!("{}", print);

    // bye bye
    Ok(())
}
