use chrono::{NaiveDate};
use scraper::{Html, Selector};
use crate::database::domain::{Movie, Screening, TheatersHtmlMap};

pub struct Scrapper {}
impl Scrapper {
    pub fn get_screenings_from_html(pages_per_theaters_per_date: TheatersHtmlMap)-> Vec<Screening> {
        let mut screenings: Vec<Screening> = Vec::new();

        for (theater, page_per_date) in pages_per_theaters_per_date {
            for (date_str, html_content) in page_per_date {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                let document = Html::parse_document(&html_content);

                let movie_blocks_selector = Selector::parse(".component--cinema-list-item").unwrap();
                let title_blocks_selector = Selector::parse(".block--title > a").unwrap();
                let screening_time_blocks_selector = Selector::parse(".screening-start").unwrap();
                let grade_blocks_selector = Selector::parse(".notes > h1").unwrap();

                for movie_block in document.select(&movie_blocks_selector) {
                    let movie_title: String = match movie_block.select(&title_blocks_selector).next() {
                        Some(title) => {
                            title.text().collect::<Vec<_>>().join("")
                        },
                        None => {
                            String::from("")
                        }
                    };

                    if movie_title.is_empty() {
                        continue;
                    }

                    let mut movie_screening_times: Vec<String> = Vec::new();
                    for screening in movie_block.select(&screening_time_blocks_selector) {
                        movie_screening_times.push(
                            screening.text().collect::<Vec<_>>().join("")
                        );
                    }

                    if movie_screening_times.is_empty() {
                        continue;
                    }

                    let movie_grade: f32 = match movie_block.select(&grade_blocks_selector).next() {
                        Some(grade) => {
                            grade.text().collect::<Vec<_>>().join("").replace(",", ".").parse::<f32>().unwrap()
                        },
                        None => {
                            0.0
                        }
                    };

                    screenings.push(Self::screenings(theater, movie_title, movie_grade, movie_screening_times, date))
                }
            }
        }

        screenings
    }

    fn screenings(theater: i8, movie_title: String, movie_grade: f32, movie_screening_times: Vec<String>, date: NaiveDate) -> Screening {
        Screening::new(
            theater,
            Movie::new(movie_title, movie_grade),
            date,
            movie_screening_times
        )
    }
}
