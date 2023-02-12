use std::{env, process};
use recho::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    recho::run(config);
}
