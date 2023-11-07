use std::env;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Missing arguments error: {err}");
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Run error: ${e}");
        std::process::exit(1);
    }
}
