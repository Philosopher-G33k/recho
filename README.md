# recho
Rust based clone of echo command

# How to build

1. clone to repository to your desired location
2. cd into the directory
3. run cargo build command to generate binary
4. debug binary will be placed in ./target/debug folder
5. to run the binary ./target/debug/reacho <your string>
6. to run directly, use the following command
  cargo run <your string>

# Project structure

Source code comprises of a binary package and a library package.
Business logic resides in the lib.rs library package.
  
# Future improvements
  
Currently the cmd line arg parsing is being done using the standard library's env::args method.
Integration of clap crate can make the process easier and provide more flexibilty.

To learn more about clap crate, please visit https://docs.rs/clap/latest/clap/
