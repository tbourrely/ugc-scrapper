use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type Theater = i16;

pub type HtmlFromTheatersByDate = HashMap<Theater, HashMap<String, String>>;

pub type MovieTitle = String;

pub type MoviesFromHtml = HashMap<MovieTitle, Movie>;

#[derive(Debug)]
pub enum PollType {
    SelectDay = 0,
    SelectMovie = 1
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: uuid::Uuid,
    pub distant_id: Option<uuid::Uuid>,
    pub poll_type: PollType,
    pub created_at: Option<NaiveDate>,
}

impl Poll {
    pub fn new(distant_id: uuid, poll_type: PollType, created_at: Option<NaiveDate>) -> Self {
        Poll {
            id: uuid::Uuid::new_v4(),
            distant_id,
            poll_type,
            created_at,
        }
    }
}

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
