# Rust Subtext

This workspace contains Rust packages and applications for working with Subtext.

The first package is a protoype recursive descent parser that attempts to
interpret a byte buffer as Subtext. The parser's only dependency is
[Tendril](https://github.com/servo/tendril), which it leverages to ergonomically
slice strings while minimizing copies.

To run tests: `cargo test`
To try the example: `cargo run --example parse examples/example.subtext`
