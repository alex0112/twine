use url::{Url, ParseError};
use regex::Regex;
use reqwest::blocking::{get, Response};


pub fn fetch_gif(url: Url) -> Result<Response, String> {
    let uid = capture_uid(&config.url)?;
    let gif_url = raw_gif_url(uid)?.to_string();

    get(gif_url)?
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
