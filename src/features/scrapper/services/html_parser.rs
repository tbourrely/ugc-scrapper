use crate::database::models::{Movie, Screening, Theater};
use crate::features::scrapper::scrapper_domain::HtmlFromTheatersByDate;
use chrono::NaiveDate;
use log::debug;
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct HtmlParser {}
impl HtmlParser {
    pub fn get_movies_from_html(
        pages_per_theaters_per_dates: &HtmlFromTheatersByDate,
    ) -> Result<Vec<Movie>, SelectorErrorKind<'static>> {
        let mut movies: HashMap<String, Movie> = HashMap::new();

        for (theater, page_per_date) in pages_per_theaters_per_dates {
            for (date_str, html_content) in page_per_date {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                let document = Html::parse_document(&html_content);

                let movie_blocks_selector = Selector::parse(".component--cinema-list-item")?;
                let title_blocks_selector = Selector::parse(".block--title > a")?;
                let screening_time_blocks_selector = Selector::parse(".screening-time-start")?;
                let screening_language_blocks_selector = Selector::parse(".screening-lang")?;
                let screening_block_selector = Selector::parse("[data-filmid]")?;
                let grade_blocks_selector = Selector::parse(".notes > h1")?;

                for movie_block in document.select(&movie_blocks_selector) {
                    let movie_title: String =
                        match movie_block.select(&title_blocks_selector).next() {
                            Some(title) => {
                                String::from(title.text().collect::<Vec<_>>().join("").trim())
                            }
                            None => String::from(""),
                        };

                    if movie_title.is_empty() {
                        debug!("Movie title not found, skipping movie block");
                        continue;
                    } else {
                        debug!("Movie title : {}", movie_title);
                    }

                    let mut movie_screening_times: HashMap<String, String> = HashMap::new();
                    for screening in movie_block.select(&screening_block_selector) {
                        let screening_time =
                            match screening.select(&screening_time_blocks_selector).next() {
                                Some(time) => {
                                    String::from(time.text().collect::<Vec<_>>().join("").trim())
                                }
                                None => {
                                    debug!("Screening time not found, skipping screening block");
                                    continue;
                                }
                            };
                        let screening_language = match screening
                            .select(&screening_language_blocks_selector)
                            .next()
                        {
                            Some(lang) => {
                                String::from(lang.text().collect::<Vec<_>>().join("").trim())
                            }
                            None => {
                                debug!("Screening language not found, skipping screening block");
                                continue;
                            }
                        };

                        movie_screening_times
                            .insert(screening_time.clone(), screening_language.clone());
                    }

                    if movie_screening_times.is_empty() {
                        debug!(
                            "No screening times found for movie {movie_title}, skipping movie block"
                        );
                        continue;
                    }

                    let movie_grade: f32 = match movie_block.select(&grade_blocks_selector).next() {
                        Some(grade) => grade
                            .text()
                            .collect::<Vec<_>>()
                            .join("")
                            .replace(",", ".")
                            .parse::<f32>()
                            .unwrap_or_else(|_e| 0.0),
                        None => {
                            debug!("Movie grade not found, setting grade to 0.0");
                            0.0
                        }
                    };

                    if !movies.contains_key(&movie_title) {
                        let movie = Movie::new(None, movie_title.clone(), movie_grade);
                        movies.insert(movie_title.clone(), movie);
                    }

                    let movie = movies.get_mut(&movie_title).unwrap();
                    movie
                        .screenings
                        .push(Self::screening(*theater, movie_screening_times, date));
                }
            }
        }
        Ok(movies.values().cloned().collect())
    }

    fn screening(
        theater: Theater,
        movie_screening_times: HashMap<String, String>,
        date: NaiveDate,
    ) -> Screening {
        debug!(
            "Theater: {:?}, Date: {}, Times: {:?}",
            theater, date, movie_screening_times
        );
        Screening::new(None, theater, date, movie_screening_times)
    }
}
