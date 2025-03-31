use crate::domain::{Movie, Screening, TheatersHtmlMap};
use chrono::{NaiveDate};
use scraper::{Html, Selector};

pub struct Scrapper {}
impl Scrapper {
    pub fn scrap_screenings_from_ugc_html_page(pages_per_theaters_per_date: TheatersHtmlMap)-> Vec<Screening> {
        let mut screenings = Vec::new();

        for (theater, page_per_date) in pages_per_theaters_per_date {
            for (date_str, html_content) in page_per_date {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();

                let document = Html::parse_document(&html_content);

                // Création des sélecteurs CSS
                let movie_blocks_selector = Selector::parse(".component--cinema-list-item").unwrap();
                let title_blocks_selector = Selector::parse(".block--title > a").unwrap();
                let screening_time_blocks_selector = Selector::parse(".screening-start").unwrap();
                let grade_blocks_selector = Selector::parse(".screening-start").unwrap();

                for movie_block in document.select(&movie_blocks_selector) {
                    let movie_title = movie_block
                        .select(&title_blocks_selector)
                        .next()
                        .map(|el| el.text().collect::<String>().trim().to_string())
                        .unwrap_or_else(|| String::from("Titre non disponible"));

                    let movie_grade = movie_block
                        .select(&grade_blocks_selector)
                        .next()
                        .map(|el| el.text().collect::<String>().trim().to_string().parse::<f32>().unwrap())
                        .unwrap_or_else(|| 0.0);

                    let movie_screening_times = movie_block
                        .select(&screening_time_blocks_selector)
                        .next()
                        .map(|el| el.text().collect::<String>().trim().to_string())
                        .unwrap_or_else(|| String::from("Horaire non disponible"));

                    screenings.push(Self::screenings(theater, movie_title, movie_grade, movie_screening_times, date))
                }
            }
        }

        screenings
    }

    fn screenings(theater: i8, movie_title: String, movie_grade: f32, movie_screening_times: String, date: NaiveDate) -> Screening {
        Screening::new(
            theater,
            Movie::new(movie_title, movie_grade, movie_screening_times),
            date,
        )
    }
}
