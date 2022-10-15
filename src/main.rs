use scraper::{Html, Selector};
use reqwest::blocking::{get};
use url::{Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let html =
    //     get("https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86")?
    //     .text()?;

    let given_url = Url::parse("https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86").unwrap();

    println!("{:#?}", given_url.as_str());
    println!("{:#?}", given_url.domain().unwrap());

    Ok(())
}

fn fetch_gif(url: Url) -> Result<String, String> {
    let html =
        get(url.as_str().to_string())?
        .text()?;

    //let gif_href = scrape_gif_url(&html).unwrap();

    Ok(html)
}

// fn scrape_gif_url(original_html: &str) -> Result<&str, Box<dyn std::error::Error>> {
//     let document = Html::parse_document(original_html);
//     let gif_link_selector = Selector::parse("td.bord:nth-child(3) > a:nth-child(1)").unwrap();
//     let anchor = document.select(&gif_link_selector).next().unwrap();
//     let href = anchor.value().attr("href").unwrap();

//     Ok(href)
// }

