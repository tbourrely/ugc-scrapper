use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Theater = i16;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PollType {
    SelectDay = 0,
    SelectMovie = 1
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: Uuid,
    pub distant_id: Uuid,
    pub poll_type: PollType,
    pub created_at: Option<NaiveDate>,
}

impl Poll {
    pub fn new(distant_id: Uuid, poll_type: PollType, created_at: Option<NaiveDate>) -> Self {
        Poll {
            id: Uuid::new_v4(),
            distant_id,
            poll_type,
            created_at,
        }
    }

    pub fn get_poll_type_number(&self) -> i16 {
        self.poll_type.clone() as i16
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub grade: f32,
    pub screenings: Vec<Screening>,
}

impl Movie {
    pub fn new(title: String, grade: f32) -> Self {
        Movie {
            id: Uuid::new_v4(),
            title,
            grade,
            screenings: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screening {
    pub id: Uuid,
    pub theater: Theater,
    pub due_date: NaiveDate,
    pub hours: Vec<String>
}

impl Screening {
    pub fn new(theater: Theater, due_date: NaiveDate, hours: Vec<String>) -> Self {
        Screening {
            id: Uuid::new_v4(),
            theater,
            due_date,
            hours
        }
    }
}
