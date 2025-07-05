use std::{ops::Not};

use chrono::NaiveDate;
use scraper::{Html, Selector};

use crate::{database, listing::Listing};

const RENT_SELECTOR: &str = r#".PropertyDetails_price__yuIqw > h6:nth-child(1)"#;
const SIZE_SELECTOR: &str =
    r#"div.PropertyDetails_row__ubfkB:nth-child(2) > div:nth-child(2) > h6:nth-child(1)"#;
const START_DATE_SELECTOR: &str =
    r#"div.PropertyDetails_row__ubfkB:nth-child(3) > div:nth-child(2) > h6:nth-child(1)"#;
const END_DATE_SELECTOR: &str =
    r#"div.PropertyDetails_row__ubfkB:nth-child(3) > div:nth-child(2) > p:nth-child(2)"#;
const LANGUAGES_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(5) > p:nth-child(2)"#;
const AGES_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(2) > p:nth-child(2)"#;
const GENDER_SELECTOR: &str = r#"div.IdealTenant_cardRow__kpy4A:nth-child(3) > p:nth-child(2)"#;
const FURNITURE_SELECTOR: &str =
    r#"div.PropertyDetails_row__ubfkB:nth-child(2) > div:nth-child(2) > p:nth-child(2)"#;
const DESCRIPTION_SELECTOR: &str = r#".CommonStyles_margin_bottom_1__FaZVq"#;

fn fetch_html(link: &str) -> Result<Html, reqwest::Error> {
    Ok(Html::parse_document(&reqwest::blocking::get(link)?.text()?))
}

/// This is meant to only take oubput from the method links_to_rooms. This is because it basically just takes the last few numbers in a link with minimal care
fn id_from_link(link: &str) -> u32 {
    link.split("-")
        .last()
        .expect("something wrong with the link")
        .parse::<u32>()
        .expect("maybe the href changed?")
}

/// Returns the links to the rooms that are not already in the database. Will return error if couldn't open the website
pub fn links_to_rooms(
    search_link: &str,
    db: &database::Database,
) -> Result<Vec<String>, reqwest::Error> {
    let doc = fetch_html(search_link)?; 
    let key = Selector::parse(r#"a[class="MuiTypography-root MuiTypography-inherit MuiLink-root MuiLink-underlineNone MuiPaper-root MuiPaper-elevation MuiPaper-rounded MuiPaper-elevation0 MuiCard-root ListingCard_root__dMVxj mui-style-i2963i"]"#).expect("Something wrong with selector");

    Ok(doc
        .select(&key)
        .filter_map(|item| {
            let link = item.attr("href").unwrap().to_string();
            db.contains(id_from_link(&link)).not().then(|| "https://kamernet.nl".to_owned() + &link)
        })
        .collect::<Vec<String>>())
}

pub fn extract_room_data(link: &str) -> Result<Listing, reqwest::Error> {
    let room = fetch_html(link)?;

    let id = id_from_link(link);
    let rent = extract_rent(&room);
    let size = extract_size(&room);
    let start_date = extract_start_date(&room);
    let end_date = extract_end_date(&room);
    // more testing with better listing needed
    let languages = extract_languages(&room);
    let (min_age, max_age) = extract_ages(&room);
    let gender = extract_gender(&room);
    let furniture = extract_furniture(&room);
    let description = extract_description(&room);

    Ok(Listing::new(
        id,
        rent,
        size,
        start_date,
        end_date,
        languages,
        min_age,
        max_age,
        gender,
        furniture,
        description,
    ))
}

fn extract_rent(room: &Html) -> u32 {
    let selector = Selector::parse(RENT_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    raw[3..].parse().unwrap()
}
fn extract_size(room: &Html) -> u32 {
    let selector = Selector::parse(SIZE_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    raw.split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap()
}
fn extract_start_date(room: &Html) -> NaiveDate {
    let selector = Selector::parse(START_DATE_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    let mut split = raw.split_ascii_whitespace();
    split.nth(1);
    NaiveDate::parse_from_str(&split.collect::<String>(), "%d%b%Y").unwrap()
}
fn extract_end_date(room: &Html) -> Option<NaiveDate> {
    let selector = Selector::parse(END_DATE_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    if raw == "Indefinite rental period" {
        None
    } else {
        let mut split = raw.split_ascii_whitespace();
        split.next();
        Some(NaiveDate::parse_from_str(&split.collect::<String>(), "%d%b%Y").unwrap())
    }
}
fn extract_languages(room: &Html) -> Vec<String> {
    let selector = Selector::parse(LANGUAGES_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    if raw == "Everyone welcome" {
        vec!["all".to_owned()]
    } else {
        raw.split(" , ")
            .map(|s| s.to_owned().to_lowercase())
            .collect()
    }
}
fn extract_ages(room: &Html) -> (u8, u8) {
    let selector = Selector::parse(AGES_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    let mut split = raw.split_ascii_whitespace();
    (
        split.next().unwrap().parse().unwrap(),
        split.nth(1).unwrap().parse().unwrap(),
    )
}
fn extract_gender(room: &Html) -> String {
    let selector = Selector::parse(GENDER_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    if raw == "Not important" {
        "any".to_owned()
    } else {
        raw.to_lowercase()
    }
}
fn extract_furniture(room: &Html) -> String {
    let selector = Selector::parse(FURNITURE_SELECTOR).expect("Something wrong with selector");
    let raw = room
        .select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html();
    raw.split_ascii_whitespace()
        .next()
        .unwrap()
        .to_owned()
        .to_lowercase()
}
fn extract_description(room: &Html) -> String {
    let selector = Selector::parse(DESCRIPTION_SELECTOR).expect("Something wrong with selector");
    room.select(&selector)
        .next()
        .expect("Something wrong with selector")
        .inner_html()
}
