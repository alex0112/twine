use scraper::{Html, Selector};
use reqwest::blocking::{get};
use url::{Url};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let given_url = Url::parse("https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86").unwrap();

    let gif_url = fetch_gif(given_url);
    println!("{:#?}", gif_url.unwrap().as_str());

    Ok(())
}

fn fetch_gif(url: Url) -> Result<String, String> {
    let html =
        get(url.as_str().to_string()).unwrap()
        .text();

    let gif_href = scrape_gif_url(&html.unwrap().to_string()).unwrap();
    let gif_url = url.join(&gif_href).unwrap();


    Ok(gif_url.to_string())
}

fn scrape_gif_url(original_html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(original_html);
    let gif_link_selector = Selector::parse("td.bord:nth-child(3) > a:nth-child(1)").unwrap();
    let anchor = document.select(&gif_link_selector).next().unwrap();
    let href = anchor.value().attr("href").unwrap().to_string();

    Ok(href)
}
