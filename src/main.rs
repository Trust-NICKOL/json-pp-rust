use serde_json::Value;
use std::io;
use std::io::Read;

fn main() -> anyhow::Result<()> {
    // Read from stdin
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_to_string(&mut buffer)?;
    // parse json
    let obj: Value = serde_json::from_str(&buffer)?;
    // pretty print
    let pretty_print = serde_json::to_string_pretty(&obj)?;
    println!("{}", pretty_print);
    // bye bye
    Ok(())
}
