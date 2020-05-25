# find-dups

Rust app to find duplicate files.

After my upgrade to Ubuntu 20.04, I saw that fslint was not available anymore.
I wondered if I could write something in Rust.

This is currently really WIP, and doesn't follow any Rust best practices, as I'm learning Rust.

## Usage 

RUST_LOG=trace cargo run /some/folder/to/analyze
