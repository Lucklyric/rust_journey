use minigrep::Config;
use std::process;
use std::{env, fs};

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {err}");
        process::exit(1);
    });

    config.run();

    println!("{},{}", config.query, config.file_path);

    let query = &args[1];

    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {},", file_path);

    let contents = fs::read_to_string(&file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    // stage two
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
