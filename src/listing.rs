use chrono::{NaiveDate};
use rusqlite::Row;
use std::str::FromStr;

#[derive(Debug)]
pub struct Listing {
    id: u32,
    rent: u32,
    size: u32,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    languages: Vec<String>,
    min_age: u8,
    max_age: u8,
    gender: String,
    furniture: String,
    description: String,
}
impl Listing {
    pub fn new(id: u32, rent: u32, size: u32, start_date: NaiveDate, end_date: Option<NaiveDate>, languages: Vec<String>, min_age: u8, max_age: u8, gender: String, furniture: String, description: String) -> Listing {
        Listing { id, rent, size, start_date, end_date, languages, min_age, max_age, gender, furniture, description }
        
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn rent(&self) -> u32{ 
        self.rent
    }
    pub fn size(&self) -> u32{ 
        self.size
    }
    pub fn start_date(&self) -> NaiveDate{ 
        self.start_date
    }
    pub fn end_date(&self) -> Option<NaiveDate>{ 
        self.end_date
    }
    pub fn languages(&self) -> Vec<String>{ 
        self.languages.clone()
    }
    pub fn min_age(&self) -> u8{ 
        self.min_age
    }
    pub fn max_age(&self) -> u8{ 
        self.max_age
    }
    pub fn gender(&self) -> String{ 
        self.gender.clone()
    }
    pub fn furniture(&self) -> String{ 
        self.furniture.clone()
    }
    pub fn description(&self) -> String{ 
        self.description.clone()
    }
}

impl Default for Listing {
    fn default() -> Self {
        Self {
            id: 403,
            rent: 123,
            size: 99,
            start_date: NaiveDate::from_str("2025-01-01").unwrap(),
            end_date: NaiveDate::from_str("2025-01-01").ok(),
            languages: vec!["english".to_string(), "dutch".to_string()],
            min_age: Default::default(),
            max_age: Default::default(),
            gender: "male".to_string(),
            furniture: "unfurnished".to_string(),
            description: "This is a tester".to_string(),
        }
    }
}

/// Unpacks the listing into a tuple that can be parsed to sql
impl From<Listing>
    for (
        u32,
        u32,
        u32,
        NaiveDate,
        Option<NaiveDate>,
        String,
        u8,
        u8,
        String,
        String,
        String,
    )
{
    fn from(value: Listing) -> Self {
        (
            value.id,
            value.rent,
            value.size,
            value.start_date,
            value.end_date,
            value.languages.join(",").to_string(),
            value.min_age,
            value.max_age,
            value.gender,
            value.furniture,
            value.description,
        )
    }
}

impl<'a> TryFrom<&'a Row<'a>> for Listing {
    type Error = rusqlite::Error;
    
    fn try_from(value: &'a Row) -> Result<Self, Self::Error> {
        Ok(Listing {
            id: value.get_unwrap("id"),
            rent: value.get_unwrap("rent"),
            size: value.get_unwrap("size"),
            start_date: value.get_unwrap("start_date"),
            end_date: value.get_unwrap("end_date"),
            languages: value.get_unwrap::<_, String>("languages").split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>(),
            min_age: value.get_unwrap("min_age"),
            max_age: value.get_unwrap("max_age"),
            gender: value.get_unwrap("gender"),
            furniture: value.get_unwrap("furniture"),
            description: value.get_unwrap("description"),
        })
    } 
}