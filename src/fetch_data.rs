use std::{fs, io::Read, ops::Not};

use reqwest::{blocking::Response};
use scraper::{Html, Selector};

use crate::database;

fn fetch_html(link: &str) -> Result<String, reqwest::Error> {
    reqwest::blocking::get(link)?.text()
}

/// This is meant to only take oupput from the method links_to_rooms. This is because it basically just takes the last few numbers in a link with minimal care
fn id_from_link(link: &str) -> u32 {
    link.split("-").last().expect("something wrong with the link").parse::<u32>().expect("maybe the href changed?")
}

/// Returns the links to the rooms that are not already in the database. Will return error if couldn't open the website
fn links_to_rooms(search_link: &str, db: &database::Database) -> Result<Vec<String>, reqwest::Error> {
    // let raw = fetch_html(search_link)?;
    let raw = fs::read_to_string("./temp.txt").unwrap();
    let doc = Html::parse_document(&raw);
    let key = Selector::parse(r#"a[class="MuiTypography-root MuiTypography-inherit MuiLink-root MuiLink-underlineNone MuiPaper-root MuiPaper-elevation MuiPaper-rounded MuiPaper-elevation0 MuiCard-root ListingCard_root__dMVxj mui-style-i2963i"]"#).unwrap();

    Ok(doc.select(&key).filter_map(|item| {
        let link = item.attr("href").unwrap().to_string();
        db.contains(id_from_link(&link)).not().then(|| link)
    }).collect::<Vec<String>>())
}
