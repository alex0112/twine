use reqwest::blocking::{get};
use url::{Url, ParseError};
use regex::Regex;
use std::io::{self, Write};
use std::env;

struct Config {
    pub url: Url
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let url_arg = args.get(1).ok_or("Please Provide a URL")?;
        if url_arg.is_empty() {
            return Err("No URL provided");
        }

        let yarn_url = Url::parse(url_arg.as_ref()).map_err(|_| "Please provide a valid URL")?;

        if !Self::matches_yarn_url(&yarn_url) {
            return Err("It appears that you have entered a URL from a site other than getyarn.io");
        }

        Ok(Config { url: yarn_url })
    }
    
    fn matches_yarn_url(url: &Url) -> bool {
        let yarn_url_regex = Regex::new(r"getyarn\.io/yarn-clip").unwrap();

        yarn_url_regex.is_match(url.as_str())
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("There was a problem processing the arguments to twine: {} ", err);

        std::process::exit(1);
    });

    let uid = capture_uid(&config.url)?;
    let gif_url = raw_gif_url(uid)?.to_string();
    let raw = get(gif_url)?.bytes()?;

    io::stdout().write_all(&raw)?;

    Ok(())
}

fn capture_uid(url: &Url) -> Result<String, String> {
    let uid_regex_cap = Regex::new(r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})")
        .map_err(|err| format!("There was an error in the creation of the yarn regex capture: {}", err))?;

    let uid = uid_regex_cap
        .captures(url.as_str())
        .and_then(|capture| capture.get(1) )
        .map_or("", |m| m.as_str());

    Ok(uid.to_string())
}

fn raw_gif_url(uid: String) -> Result<Url, ParseError> {
    let raw_gif_path = "https://y.yarn.co/".to_owned() + &uid + "_text.gif";
    let raw_gif_url = Url::parse(&raw_gif_path)?;

    Ok(raw_gif_url)
}
