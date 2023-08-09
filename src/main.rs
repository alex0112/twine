// use reqwest::blocking::{get};
// use url::{Url, ParseError};
// use regex::Regex;

use lib;

use std::io::{self, Write};
use std::env;

// Twine Specific
//mod lib;

use lib::twine::config::Config;
use lib::twine::yarn::fetch_gif;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("There was a problem processing the arguments to twine: {} ", err);

        std::process::exit(1);
    });

    let raw = fetch_gif(&config.url).bytes()?;

    io::stdout().write_all(&raw)?;

    Ok(())
}
