use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

const CONFLUENCE_THEATER: i16 = 36;
const ASTORIA_THEATER: i16 = 33;
const PART_DIEU_THEATER: i16 = 58;
const CITE_INTERNATIONAL_THEATER: i16 = 32;
pub const LYON_THEATERS: [i16; 4] = [CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER];

pub type Theater = i16;

pub type HtmlFromTheatersByDate = HashMap<i16, HashMap<String, String>>;

pub type MovieTitle = String;

pub type MoviesFromHtml = HashMap<MovieTitle, Movie>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: uuid::Uuid,
    pub title: String,
    pub grade: f32,
    pub screenings: Vec<Screening>,
}

impl Movie {
    pub fn new(title: String, grade: f32) -> Self {
        Movie {
            id: uuid::Uuid::new_v4(),
            title,
            grade,
            screenings: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hours {
    pub hours: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screening {
    pub id: uuid::Uuid,
    pub theater: Theater,
    pub due_date: NaiveDate,
    pub hours: Hours
}

impl Screening {
    pub fn new(theater: Theater, due_date: NaiveDate, hours: Hours) -> Self {
        Screening {
            id: uuid::Uuid::new_v4(),
            theater,
            due_date,
            hours
        }
    }
}
