pub mod config;
mod search;

use std::{error::Error, fs};

pub use config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    let lines = if config.ignore_case {
        search::case_insensitive(&config.query, &contents)
    } else {
        search::case_sensitive(&config.query, &contents)
    };

    for line in lines {
        println!("{line}");
    }

    if contents.len() == 0 {
        println!("No results found");
    }

    Ok(())
}
