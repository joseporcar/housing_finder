use lettre::transport::smtp::commands::Data;
use scraper::{Html, Selector};

mod send_email;
use crate::send_email::Mailer;
mod database;
mod listing;

mod fetch_data;

fn main() {
    dotenvy::dotenv().expect("There is an issue with the .env file");
    // let mailer = Mailer::new();

    // mailer.send_email("there is housing in your area");

    let some_listing = listing::Listing::default();
    let manager = database::Database::open_database();
    
    // manager.add_listing(some_listing);
    // let id = 403;
    // if let Err(rusqlite::Error::QueryReturnedNoRows) = dbg!(manager.get_listing(id)) {
    //     eprintln!("listing by id {id} not found")
    // }

    //let aa = fetch_data::ids_from_links(&fetch_data::links_to_rooms("https://kamernet.nl/en/for-rent/properties-eindhoven?maxRent=8&radius=7&pageNo=1").unwrap());
    //println!("{:?}", aa)
}
