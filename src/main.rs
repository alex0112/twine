use clap::Parser;
use regex::Regex;
use url::{Url};
use std::sync::LazyLock;
use anyhow::{Context, Result, ensure};

// LazyLock bc Regex::new needs to evaluate at run time and static makes it require the Sync trait
static YARN_REGEX: LazyLock<Regex>        = LazyLock::new(|| { Regex::new(r"getyarn\.io/yarn-clip").expect("getyarn.io validation regex creation failed.") });
static UID_CAPTURE_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})").expect("uid capture regex creation failed") });

#[derive(Parser)]
struct Args {
    /// A valid yarn URL pointing to the clip your heart desires
    #[arg(required = true)]
    url: String
}

struct Twine {
    args: Args,
    url: Url,
}

impl Twine {
    fn new(args: Args) -> Result<Twine> {

        let yarn_url = Url::parse(&args.url).context("Unable to parse the provided URL")?;

        ensure!(Self::valid_yarn_url(&yarn_url), "It appears that you have entered a URL from a site other than getyarn.io");

        Ok(Twine { url: yarn_url, args })
    }

    fn valid_yarn_url(url: &Url) -> bool {
        YARN_REGEX.is_match(url.as_str())
    }

    fn capture_uid(url: &Url) -> Result<String> {

        let uid: String = UID_CAPTURE_REGEX
            .captures(url.as_str()).context("Unable to capture a getyarn uid from the provided URL")?
            .get(1).context("Unable to capture (group 1) from getyarn string")?
            .as_str()
            .to_string();

        Ok(uid)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

// TODO:
// - arboard crate (for putting the gif in clipboard) https://crates.io/crates/arboard
// - gif crate (for trimming off the end of the gif) https://crates.io/crates/gif
// - atty (for detecting whether we're writing to stdout or using `>`) https://docs.rs/atty/latest/atty/

#[cfg(test)]
mod tests {
    use super::*;

    ////////////////
    // Twine::new //
    ////////////////

    #[test]
    fn test_new_sanity() { // sanity case
        let test_url: String = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let test_args: Args = Args {
            url: test_url
        };

        assert!(Twine::new(test_args).is_ok(), "A basic call to Twine::new should return an Ok()");
    }

    #[test]
    fn test_new_with_invalid_url_in_args() {
        let invalid = "https://example.com".to_string();
        let invalid_args: Args = Args {
            url: invalid
        };

        assert!(Twine::new(invalid_args).is_err(),"Creating args with an invalid yarn URL should fail");
    }

    #[test]
    fn test_new_with_valid_url_in_args() {
        let valid = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let valid_args = Args {
            url: valid
        };

        assert!(Twine::new(valid_args).is_ok())
    }

    ///////////////////////////
    // Twine::valid_yarn_url //
    ///////////////////////////

    #[test] // TODO: this regex could be pushed a bit more in terms of edge cases
    fn test_valid_yarn_url_invalid_url_fails() { 
        let invalid = Url::parse("https://example.com").unwrap();
        assert!(!Twine::valid_yarn_url(&invalid), "example.com should not be considered a valid yarn url");
    }
    
    #[test]
    fn test_valid_yarn_url_valid_url_succeeds() {
        let valid = Url::parse("https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138").unwrap();
        assert!(Twine::valid_yarn_url(&valid), "The example clip should be considered a valid URL");
    }

    /////////////////////////////
    // Twine::test_capture_uid //
    /////////////////////////////

    #[test]
    fn test_capture_uid_invalid_url() {
        let invalid = Url::parse("https://example.com").unwrap();
        assert!(Twine::capture_uid(&invalid).is_err(), "example.com should not produce a uid");
    }

    #[test]
    fn test_capture_uid_valid_url() {
        let valid = Url::parse("https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138").unwrap();
        let expected_uid = "bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let uid = Twine::capture_uid(&valid);
        
        assert!(uid.is_ok(), "a valid yarn url should produce a uid");
        assert_eq!(uid.unwrap(), expected_uid, "a valid yarn url should produce a uid");
    }
}

// haiku:
// eight bits per pixel
// bitmap image format, and
// pronunciation
