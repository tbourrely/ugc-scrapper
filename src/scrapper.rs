use crate::domain::{Content, Movie, Screening};
use chrono::NaiveDate;

#[allow(dead_code)]
struct HttpAgent {}
impl HttpAgent {
    #[allow(dead_code)]
    fn page(_movie: Movie, _date: NaiveDate) -> Content {
        "".to_string()
    }
}

#[allow(dead_code)]
struct Scrapper {}
impl Scrapper {
    #[allow(dead_code)]
    fn screenings(_content: Content) -> Vec<Screening> {
        vec![Screening::new(
            0,
            Movie::new(),
            NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap(),
        )]
    }
}
