use crate::{database::Database, listing::Listing, send_email::Mailer};

pub mod database;
mod fetch_data;
mod listing;
pub mod send_email;

fn send_email(mailer: &Mailer, listing: &Listing) {
    let content = format!(
        "Hey! There is a new listing with a cost of: €{}\n
    It measures {}m2\n
    It is availible from {} to {}\n
    It is {}\n
    Here are words from the lanlord:
     > {}
    ",
        listing.rent(),
        listing.size(),
        listing.start_date(),
        listing
            .end_date()
            .map_or("indefinite".to_owned(), |date| date.to_string()),
        listing.furniture(),
        listing.description()
    );
    mailer.send_email(&content);
}

/// Main function.
pub fn refresh_data(link: &str, database: Database, mailer: Mailer) {
    let Ok(links) = fetch_data::links_to_rooms(link, &database) else {
        println!(
            "Couldn't fetch the search link at time {}",
            chrono::Local::now()
        );
        return;
    };
    for link in links {
        let Ok(listing) = fetch_data::extract_room_data(&link) else {
            println!(
                "Couldn't extract data from {} at time {}",
                link,
                chrono::Local::now()
            );
            continue;
        };
        if listing.is_valid() {
            send_email(&mailer, &listing);
        }

        database.add_listing(listing);
    }
}
