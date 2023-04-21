use std::env;
use std::process;
use minigrep::Config;

// cargo run to src/poem.txt
// CASE_INSENSITIVE=1 cargo run to src/poem.txt
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    print!("Searching for {} ", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}
