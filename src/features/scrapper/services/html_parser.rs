use std::collections::HashMap;
use chrono::{NaiveDate};
use scraper::{Html, Selector};
use scraper::error::SelectorErrorKind;
use crate::database::models::{Movie, Screening, Theater};
use crate::features::scrapper::scrapper_domain::HtmlFromTheatersByDate;

pub struct HtmlParser {}
impl HtmlParser {
    pub fn get_movies_from_html(pages_per_theaters_per_dates: &HtmlFromTheatersByDate) -> Result<Vec<Movie>, SelectorErrorKind<'static>> {
        let mut movies: HashMap<String, Movie> = HashMap::new();

        for (theater, page_per_date) in pages_per_theaters_per_dates {
            for (date_str, html_content) in page_per_date {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                let document = Html::parse_document(&html_content);

                let movie_blocks_selector = Selector::parse(".component--cinema-list-item")?;
                let title_blocks_selector = Selector::parse(".block--title > a")?;
                let screening_time_blocks_selector = Selector::parse(".screening-start")?;
                let grade_blocks_selector = Selector::parse(".notes > h1")?;

                for movie_block in document.select(&movie_blocks_selector) {
                    let movie_title: String = match movie_block.select(&title_blocks_selector).next() {
                        Some(title) => {
                            String::from(title.text().collect::<Vec<_>>().join("").trim())
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
                            String::from(screening.text().collect::<Vec<_>>().join("").trim())
                        );
                    }

                    if movie_screening_times.is_empty() {
                        continue;
                    }

                    let movie_grade: f32 = match movie_block.select(&grade_blocks_selector).next() {
                        Some(grade) => {
                            grade.text().collect::<Vec<_>>().join("").replace(",", ".").parse::<f32>().unwrap_or_else(|_e| 0.0)
                        },
                        None => {
                            0.0
                        }
                    };
                    
                    if !movies.contains_key(&movie_title) {
                        let movie = Movie::new(movie_title.clone(), movie_grade);
                        movies.insert(movie_title.clone(), movie);
                    }

                    let movie = movies.get_mut(&movie_title).unwrap();
                    movie.screenings.push(
                        Self::screenings(*theater, movie_screening_times, date)
                    );
                }
            }
        }
        Ok(movies.values().cloned().collect())
    }

    fn screenings(theater: Theater, movie_screening_times: Vec<String>, date: NaiveDate) -> Screening {
        Screening::new(
            theater,
            date,
            movie_screening_times
        )
    }
}
