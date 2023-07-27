use reqwest::blocking::{get};
use url::{Url, ParseError};
use regex::Regex;
use std::io::{self, Write};
use std::env;

/////////////////////////////////////////////////////////////////////
// https://y.yarn.co/84e0913e-9df9-4e44-90dd-e25a079bae86_text.gif //
/////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //    let given_url = Url::parse("https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86").unwrap();


    let args: Vec<String> = env::args().collect();
    let given_url = Url::parse(&args[1]).expect("Either the URL provided was invalid or not provided");

    assert!(is_yarn_url(&given_url));
    let uid = capture_uid(&given_url).unwrap();
    let gif_url = raw_gif_url(uid).unwrap().to_string();
    let raw = get(gif_url).unwrap().bytes().unwrap();

    io::stdout().write_all(&raw)?;

    Ok(())
}

fn is_yarn_url(url: &Url) -> bool {
    let yarn_url_regex = Regex::new(r"getyarn\.io/yarn-clip").unwrap();

    return yarn_url_regex.is_match(url.as_str());
}

fn capture_uid(url: &Url) -> Result<String, String> {
    let uid_regex_cap = Regex::new(r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})").unwrap();
    let uid = uid_regex_cap.captures(url.as_str()).unwrap().get(1).map_or("", |m| m.as_str());

    Ok(uid.to_string())
}

fn raw_gif_url(uid: String) -> Result<Url, ParseError> {
    let raw_gif_path = "https://y.yarn.co/".to_owned() + &uid + "_text.gif";
    let raw_gif_url = Url::parse(&raw_gif_path).unwrap();

    Ok(raw_gif_url)
}
