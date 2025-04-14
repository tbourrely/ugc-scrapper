use std::collections::HashMap;
use chrono::NaiveDate;

pub type Theater = i8;
pub type Content = String;

pub type TheatersHtmlMap = HashMap<i8, HashMap<String, String>>;

#[derive(Debug)]
pub struct Movie {
    pub id: uuid::Uuid,
    pub title: String,
    pub grade: f32
}

impl Movie {
    pub fn new(title: String, grade: f32) -> Self {
        Movie {
            id: uuid::Uuid::new_v4(),
            title,
            grade,
        }
    }
}

#[derive(Debug)]
pub struct Screening {
    pub id: uuid::Uuid,
    pub theater: Theater,
    pub movie: Movie,
    pub due_date: NaiveDate,
    pub hours: Vec<String>
}

impl Screening {
    pub fn new(theater: Theater, movie: Movie, due_date: NaiveDate, hours: Vec<String>) -> Self {
        Screening {
            id: uuid::Uuid::new_v4(),
            theater,
            movie,
            due_date,
            hours
        }
    }
}
