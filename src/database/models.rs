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
    pub fn new(id: Option<Uuid>, title: String, grade: f32) -> Self {
        let uuid: Uuid;
        if id.is_none() {
            uuid = Uuid::new_v4();
        } else {
            uuid = id.unwrap();
        }

        Movie {
            id: uuid,
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
    pub fn new(id: Option<Uuid>, theater: Theater, due_date: NaiveDate, hours: Vec<String>) -> Self {
        let uuid: Uuid;
        if id.is_none() {
            uuid = Uuid::new_v4();
        } else {
            uuid = id.unwrap();
        }
        Screening {
            id: uuid,
            theater,
            due_date,
            hours
        }
    }
}

#[derive(Debug)]
pub struct MoviesSeen {
    pub id: Uuid,
    pub content: String,
    pub created_at: Option<NaiveDate>,
}

impl MoviesSeen {
    pub fn new(id: Option<Uuid>, content: String, created_at: Option<NaiveDate>) -> Self {
        let uuid: Uuid;
        if id.is_none() {
            uuid = Uuid::new_v4();
        } else {
            uuid = id.unwrap();
        }
        MoviesSeen {
            id: uuid,
            content,
            created_at
        }
    }
}