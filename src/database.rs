
use rusqlite::{Connection, named_params};

use crate::listing::Listing;

pub struct Database {
    connection: Connection,
}

impl Database {
    fn create_database(conn: &Connection) {
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS listings (
                id INTEGER PRIMARY KEY,
                rent INTEGER,
                size INTEGER,
                start_date TEXT,
                end_date TEXT,
                languages TEXT,
                min_age INTEGER,
                max_age INTEGER,
                gender TEXT,
                furniture TEXT,
                description TEXT)",
            [],
        )
        .unwrap();
    }

    pub fn open_database() -> Database {
        let conn = rusqlite::Connection::open("./data.db").unwrap();
        if !conn.table_exists(None, "listing").unwrap() {
            Self::create_database(&conn);
        }

        Database { connection: conn }
    }

    pub fn add_listing(&self, listing: Listing) {
        self.connection.execute("INSERT INTO listings (id, rent, size, start_date, end_date, languages, min_age, max_age, gender, furniture, description) VALUES (:id, :rent, :size, :start_date, :end_date, :languages, :min_age, :max_age, :gender, :furniture, :description)",
            named_params! {":id": listing.id(),
                        ":rent": listing.rent(),
                        ":size": listing.size(),
                        ":start_date": listing.start_date(),
                        ":end_date": listing.end_date(),
                        ":languages": listing.languages().join(",").to_string(),
                        ":min_age": listing.min_age(),
                        ":max_age": listing.max_age(),
                        ":gender": listing.gender(),
                        ":furniture": listing.furniture(),
                        ":description": listing.description()}).unwrap();
    }

    pub fn get_listing(&self, id: u32) -> Result<Listing, rusqlite::Error> {
        self.connection.query_row_and_then("SELECT * FROM listings WHERE id=?1", [id], |row| Listing::try_from(row))
    }
}
