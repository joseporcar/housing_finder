use std::{fs, io::Read, ops::Not};

use reqwest::{blocking::Response};
use scraper::{Html, Selector};

use crate::{database, listing::Listing};

const ROOM_SELECTOR: &str = r#".PropertyDetails_price__yuIqw > h6:nth-child(1)"#;
const SIZE_SELECTOR: &str = r#"div.PropertyDetails_row__ubfkB:nth-child(2) > div:nth-child(2) > h6:nth-child(1)"#;
const START_DATE_SELECTOR: &str = r#"div.PropertyDetails_row__ubfkB:nth-child(3) > div:nth-child(2) > h6:nth-child(1)"#;
const END_DATE_SELECTOR: &str = r#"div.PropertyDetails_row__ubfkB:nth-child(3) > div:nth-child(2) > p:nth-child(2)"#;
const LANGUAGES_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(5) > p:nth-child(2)"#;
const AGES_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(2) > p:nth-child(2)"#;
const GENDER_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(3) > p:nth-child(2)"#;
const FURNITURE_SELECTOR: &str = r#"div.PropertyDetails_row__ubfkB:nth-child(2) > div:nth-child(2) > p:nth-child(2)"#;
const DESCRIPTION_SELECTOR: &str = r#".CommonStyles_margin_bottom_1__FaZVq"#;


fn fetch_html(link: &str) -> Result<Html, reqwest::Error> {
    Ok(Html::parse_document(&reqwest::blocking::get(link)?.text()?))
}

/// This is meant to only take oubput from the method links_to_rooms. This is because it basically just takes the last few numbers in a link with minimal care
fn id_from_link(link: &str) -> u32 {
    link.split("-").last().expect("something wrong with the link").parse::<u32>().expect("maybe the href changed?")
}

/// Returns the links to the rooms that are not already in the database. Will return error if couldn't open the website
fn links_to_rooms(search_link: &str, db: &database::Database) -> Result<Vec<String>, reqwest::Error> {
    // let doc = fetch_html(search_link)?; TODO uncomment
    let doc = Html::parse_document(&fs::read_to_string("./temp.txt").unwrap());
    let key = Selector::parse(r#"a[class="MuiTypography-root MuiTypography-inherit MuiLink-root MuiLink-underlineNone MuiPaper-root MuiPaper-elevation MuiPaper-rounded MuiPaper-elevation0 MuiCard-root ListingCard_root__dMVxj mui-style-i2963i"]"#).expect("Something wrong with selector");

    Ok(doc.select(&key).filter_map(|item| {
        let link = item.attr("href").unwrap().to_string();
        db.contains(id_from_link(&link)).not().then(|| link)
    }).collect::<Vec<String>>())
}

pub fn extract_room_data(link: &str) -> Result<Listing, reqwest::Error> {
    // let room = fetch_html(link)?; TODO uncomment
    let room = Html::parse_document(&fs::read_to_string("./sample.txt").unwrap());
    let rent = extract_rent(&room);
    // let id = id_from_link(link);
    println!("{rent}");

    // Ok(Listing::new(id, rent, size, start_date, end_date, languages, min_age, max_age, gender, furniture, description))
    Ok(Listing::default())
}

fn extract_rent(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_size(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_start_date(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_end_date(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_languages(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_ages(room: &Html) -> (u8, u8) {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    //raw[1..].parse().unwrap();
    todo!()
}
fn extract_gender(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_furniture(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}
fn extract_description(room: &Html) -> u32 {
    let selector = Selector::parse(ROOM_SELECTOR).expect("Something wrong with selector");
    let raw = room.select(&selector).next().expect("Something wrong with selector").inner_html();
    raw[1..].parse().unwrap()
}

