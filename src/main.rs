use reqwest::blocking::{get};
use url::{Url, ParseError};
use regex::Regex;

/////////////////////////////////////////////////////////////////////
// https://y.yarn.co/84e0913e-9df9-4e44-90dd-e25a079bae86_text.gif //
/////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let given_url = Url::parse("https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86").unwrap();

    assert!(is_yarn_url(&given_url));
    let uid = capture_uid(&given_url).unwrap();
    let gif_url = raw_gif_url(uid).unwrap().to_string();
    let raw = get(gif_url).unwrap().text();

    print!("{:#?}", raw);

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

// fn fetch_gif(given_url: &Url) -> Result<Url, ParseError> {
//     let gif_page_url = scrape_gif_page_url(given_url).unwrap();
// //    println!("{:#?}", gif_page_url.as_str());
//     let raw_gif_url = scrape_raw_gif_url(&gif_page_url);



//     return raw_gif_url;
// }

// // Given a plain ol' yarn link (like https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86)
// // Find the page representing the gif version of the clip.
// //
// // TODO: This could be achieved more easily by appending "/gif" to the existing link if it does not exist
// // do the concatenation instead of scraping, faster that way and we can save ourselves a network request.
// fn scrape_gif_page_url(given_url: &Url) -> Result<Url, ParseError> {
//     let gif_href_list = scrape_select(given_url, "td.bord:nth-child(3) > a:nth-child(1)", "href").unwrap();
// //    println!("{:#?}", given_url.as_str());

//     let gif_href = &gif_href_list[0];
//     let gif_page_url = given_url.join(&gif_href);

//     return gif_page_url;
// }

// // Given the url of the gif page (e.g. https://getyarn.io/yarn-clip/84e0913e-9df9-4e44-90dd-e25a079bae86/gif)
// // find the <img src="..."> element attribute pointing to the raw gif file itself.
// fn scrape_raw_gif_url(gif_page_url: &Url) -> Result<Url, ParseError> {
//     let img_src_list = scrape_select(&gif_page_url, ".img-resp", "src").unwrap();
//     let img_src = &img_src_list[1];
//     let raw_gif_url = Url::parse(&img_src);

//     return raw_gif_url;
// }

// // Given a url struct, a css selector string, and an attribute,
// // fetch that page, scrape using the selector, and return a vector of all matched
// // elements and that particular attribute.
// //
// // e.g. scrape_select(url, "img", "src")
// // grabs the src of every image on the page.
// fn scrape_select(url: &Url, selector: &str, attr: &str) -> Result<Vec<String>, ParseError> {
//     let html =
//         get(url.as_str().to_string()).unwrap()
//         .text().unwrap();

//     let document = Html::parse_document(&html);
//     let selector = Selector::parse(selector).unwrap();

//     let elements = document.select(&selector);
//     let attributes =
//         elements
//         .map( |element| element.value().attr(&attr).unwrap().to_string() )
//         .collect();

//     println!("{:#?}", attributes);

//     Ok(attributes)
// }
