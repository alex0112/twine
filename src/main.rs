use clap::Parser;
use regex::Regex;
use url::{Url};
use std::sync::LazyLock;
use anyhow::{Context, Result, ensure};
use minreq;
use std::io::{self, Write};

// LazyLock bc Regex::new needs to evaluate at run time and static makes it require the Sync trait
static YARN_REGEX: LazyLock<Regex>        = LazyLock::new(|| { Regex::new(r"getyarn\.io/yarn-clip").expect("getyarn.io validation regex creation failed.") });
static UID_CAPTURE_REGEX: LazyLock<Regex> = LazyLock::new(|| { Regex::new(r"([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})").expect("uid capture regex creation failed") });

#[derive(Parser)]
struct Args {
    /// A valid yarn URL
    #[arg(required = true)]
    url: String
}

#[derive(Debug)]
struct Uid(String);

impl Uid {
    fn new(raw_uid: String) -> Result<Uid> {
        ensure!(UID_CAPTURE_REGEX.is_match(&raw_uid), "Invalid UID");

        Ok(Uid(raw_uid.to_string()))
    }

    fn as_str(&self) -> &str {
        &self.0
    }

    fn from_url(url: &Url) -> Result<Uid> {
        let uid: String = UID_CAPTURE_REGEX
            .captures(url.as_str()).context("Unable to capture a getyarn uid from the provided URL")?
            .get(1).context("Unable to capture (group 1) from getyarn string")?
            .as_str()
            .to_string();

        Ok(Uid(uid))
    }
}

impl PartialEq for Uid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct Twine {
    args: Args,
    url: Url,
    uid: Uid,
    file_url: Url
}

impl Twine {
    fn new(args: Args) -> Result<Twine> {

        let url = Url::parse(&args.url).context("Unable to parse the provided URL")?;

        ensure!(Self::valid_yarn_url(&url), "It appears that you have entered a URL from a site other than getyarn.io");

        let uid: Uid = Uid::from_url(&url).context("Unable to generate uid from provided URL")?;
        let file_url: Url = Self::raw_gif_url(&uid).context("Unable to generate raw file url")?;

        dbg!(&file_url.as_str());

        Ok(Twine { url, uid, args, file_url })
    }

    fn valid_yarn_url(url: &Url) -> bool {
        YARN_REGEX.is_match(url.as_str())
    }

    fn raw_gif_url(uid: &Uid) -> Result<Url> {
        let raw_gif_path = "https://y.yarn.co/".to_owned() + uid.as_str() + "_text.gif";
        let raw_gif_url = Url::parse(&raw_gif_path)?;

        Ok(raw_gif_url)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let twine = Twine::new(args)?;

    let response = minreq::get(twine.file_url).send()?;
    let raw: &[u8] = response.as_bytes();

    io::stdout().write_all(raw)?;
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

        assert!(Twine::new(invalid_args).is_err(), "Creating args with an invalid yarn URL should fail");
    }

    #[test]
    fn test_new_with_valid_url_in_args() {
        let valid = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let valid_args = Args {
            url: valid
        };

        assert!(Twine::new(valid_args).is_ok())
    }

    #[test]
    fn test_new_should_correctly_parse_url() {
        let valid = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138";
        let expected = Url::parse(valid).expect("Unable to create test url");
        
        let args: Args = Args {
            url: valid.to_string()
        };

        let twine: Twine = Twine::new(args).expect("Unable to create test Twine struct");

        assert_eq!(twine.url, expected, "Correctly parses the url");
    }

    #[test]
    fn test_new_should_correctly_parse_Uid() {
        let valid = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let args: Args = Args {
            url: valid
        };

        let expected_uid = Uid::new("bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string()).expect("Unable to create test Uid");
        let twine = Twine::new(args).expect("Unable to create test Twine struct from args");

        assert_eq!(twine.uid, expected_uid, "A twine struct should correctly get the Uid from its arguments");

    }

    ///////////////////////////
    // Twine::valid_yarn_url //
    ///////////////////////////

    #[test] // TODO: this regex could be pushed a bit more in terms of edge cases etc.
    fn test_valid_yarn_url_invalid_url_fails() { 
        let invalid = Url::parse("https://example.com").unwrap();
        assert!(!Twine::valid_yarn_url(&invalid), "example.com should not be considered a valid yarn url");
    }
    
    #[test]
    fn test_valid_yarn_url_valid_url_succeeds() {
        let valid = Url::parse("https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138").unwrap();
        assert!(Twine::valid_yarn_url(&valid), "The example clip should be considered a valid URL");
    }

    ////////////////////////
    // Twine::raw_gif_url //
    ////////////////////////

    // #[test]
    // fn test_raw_gif_url_invalid_uid() {
    //     let invalid = "sphinx of black quartz, judge my vow".to_string();
    //     let url = Twine::raw_gif_url(invalid);

    //     assert!(url.is_err(), "An invalid uid string should produce an error");
    // }

    #[test]
    fn test_raw_gif_url_valid_uid() {
        let valid = Uid("bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string());
        let expected = Url::parse("https://y.yarn.co/bbdb6c42-1fa4-44a5-8728-07529eafb138_text.gif").expect("Unable to set up expected yarn raw url for test");
        
        assert_eq!(Twine::raw_gif_url(&valid).unwrap(), expected, "should be able to produce a valid url for the raw gif file");
    }

    //////////////
    // Uid::new //
    //////////////

    #[test]
    fn test_uid_new_string_invalid() {
        let invalid = "sphinx of black quartz judge my vow".to_string();
        assert!(Uid::new(invalid).is_err(), "A non-uid string should not allow for creation of a uid");
    }

    #[test]
    fn test_uid_new_string_valid() {
        let valid = "bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();

        assert!(Uid::new(valid).is_ok(), "A uid string should allow for the creation of a uid");
    }

    #[test]
    fn test_uid_from_url_invalid() {
        let invalid = Url::parse("https://example.com/foo/bar").expect("Unable to set up a valid url for example.com");
        assert!(Uid::from_url(&invalid).is_err(), "A url not containing a UID string should produce an error");
    }

    #[test]
    fn test_uid_from_url_valid() {
        let valid = Url::parse("https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138").expect("Unable to create test URL");
        let expected_uid = Uid::new("bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string()).expect("Unable to create test Uid");
        let actual_uid = Uid::from_url(&valid).expect("Did not properly create Uid for test");

        assert_eq!(actual_uid, expected_uid, "A url not containing a UID string should produce an error");
    }
}

// haiku:
// eight bits per pixel
// bitmap image format, and
// pronunciation
