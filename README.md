# json-pp-rust

Faster alternative to `json-pp` written in Rust, using serde-json. 

Reads a string from `stdin` parses it as json and prints it out as a pretty json string.

## Installation
    
    cargo install json-pp-rust

## Usage

    $ echo "{\"hello\": [\"world\", \"json\", 10]}" | json-pp-rust
    {
      "hello": [
        "world",
        "json",
        10
      ]
    }
