use std::collections::HashMap;
use chrono::NaiveDate;

pub type Theater = i8;
pub type Content = String;

type DatesMap = HashMap<String, Vec<String>>;
type MoviesMap = HashMap<String, DatesMap>;
pub type TheatersHtmlMap = HashMap<i8, HashMap<String, String>>;

pub struct Movie {
    pub title: String,
    pub grade: f32,
    pub synopsis: String,
}

impl Movie {
    pub fn new(title: String, grade: f32, synopsis: String) -> Self {
        Movie {
            title,
            grade,
            synopsis,
        }
    }
}

pub struct Screening {
    pub id: uuid::Uuid,
    pub theater: Theater,
    pub movie: Movie,
    pub due_date: NaiveDate,
}

impl Screening {
    pub fn new(theater: Theater, movie: Movie, due_date: NaiveDate) -> Self {
        Screening {
            id: uuid::Uuid::new_v4(),
            theater,
            movie,
            due_date,
        }
    }
}
