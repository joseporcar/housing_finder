
use std::{thread, time};

use dotenvy::var;
use housing_finder::{database, refresh_data, send_email::Mailer};
use rand::Rng;


fn main() {
    // the first test must print all of the database, and it must send the emails.

    dotenvy::dotenv().expect("There is an issue with the .env file");

    let mailer = Mailer::new();

    let manager = database::Database::open_database();

    let mut seed = rand::rng();

    loop {
        refresh_data(
        &var("SEARCH_LINK").expect("error getting search link from dotenv"),
        &manager,
        &mailer,);
        thread::sleep(time::Duration::from_secs(300 + seed.random_range(0..600)));
    }
    

}
