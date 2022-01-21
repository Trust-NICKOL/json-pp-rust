# json-pp-rust

Faster alternative to `json_pp` written in Rust, using `serde-json`. 

Reads a string from `stdin` parses it as json and prints it out as a pretty json string.

## Installation Notes

### Requirements

- Rust _(2021 edition)_
- Cargo

### Rust

We suggest that you install Rust using the 'rustup' tool. Rustup will install
the latest version of Rust, Cargo, and the other binaries used for development.

Follow the instructions at [Installing
Rust](https://www.rust-lang.org/tools/install).

For Mac users, Homebrew is also an option.  The Mac Homebrew command is `brew
install rustup` and then `rustup-init`. See [Mac
Setup](https://sourabhbajaj.com/mac-setup/Rust/) & [Installing
Rust](https://www.rust-lang.org/tools/install) for more details.

After installation, you should have `rustc`, `cargo`, & `rustup`. You should
also have `~/.cargo/bin` in your PATH environment variable.

### Installation using cargo

You can easily install the latest published version of `json-pp-rust` with `cargo`.

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

## Contributing

### How to build the software

Cargo is used as the package manager and build system for `json-pp-rust`.

    $ git clone https://github.com/Trust-NICKOL/json-pp-rust
    $ cd json-pp-rust
    $ cargo build --release
    $ ./target/release/json-pp-rust --version


### How to test the software

The unit-tests of this repository can be used to test the functionality of this library.

    cargo test

### Known issues

There are currently no know issues with this tool.

### Getting help

Please you the issue tracker of the github repository if you have any problems using the library.

### Getting involved

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

For more information see [CONTRIBUTING](CONTRIBUTING.md).

### License

This software is released under the MIT License.