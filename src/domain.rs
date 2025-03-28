use chrono::NaiveDate;

pub type Theater = i8;
pub type Content = String;

pub struct Movie {
    pub title: String,
    pub grade: f32,
    pub synopsis: String,
}

impl Movie {
    pub fn new() -> Self {
        Movie {
            title: "".to_string(),
            grade: 0.0,
            synopsis: "".to_string(),
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
