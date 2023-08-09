use url::{Url, ParseError};
use regex::Regex;

pub struct Config {
    pub url: Url,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
