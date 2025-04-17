use clap::Parser;
use url::{Url};

#[derive(Parser)]
struct Args {
    /// A valid yarn URL pointing to the clip your heart desires
    #[arg(required = true)]
    url: String
}

// A haiku about GIFs:
// eight bits per pixel
// bitmap image format, and
// pronunciation

struct Twine {
    url: Url,
}

impl Twine {
    fn new(args: Args) -> Result<Twine, &'static str> {

        let yarn_url = Url::parse(&args.url).map_err(|_| "Please provide a valid URL")?;

        // if !Self::matches_yarn_url(&yarn_url) {
        //     return Err("It appears that you have entered a URL from a site other than getyarn.io");
        // }

        Ok(Twine { url: yarn_url })
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

    #[test]
    fn test_new() { // sanity case
        let test_url: String = "https://getyarn.io/yarn-clip/bbdb6c42-1fa4-44a5-8728-07529eafb138".to_string();
        let test_args: Args = Args {
            url: test_url
        };

        assert!(Twine::new(test_args).is_ok(), "A basic call to Twine::new should return an Ok()");
    }
}
