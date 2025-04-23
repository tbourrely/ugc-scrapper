use chrono::NaiveDate;
use sqlx::PgPool;
use crate::database::domain::Theater;
use crate::database::movie::init_movie_repository;
use crate::features::scrapper::services::html_parser::HtmlParser;
use crate::features::scrapper::services::ugc::Ugc;
use crate::features::scrapper::utils::{dates, theaters};

pub async fn retrieve_movies_from_ugc (db: &PgPool) {
    let dates: Vec<NaiveDate> = dates::get_each_dates_of_current_week(None);
    let theaters: Vec<Theater> = theaters::get_lyon_theaters();

    let html_per_theaters_per_dates = match Ugc::get_html_from_theaters_per_dates(theaters, dates)
        .await {
        Ok(p) => {
            println!("Successfully retrieved html from each theater on ugc web site");
            p
        },
        Err(e) => { panic!("An error occurred while using migrations files: {e:?}") }
    };

    let movies = match HtmlParser::get_movies_from_html(&html_per_theaters_per_dates) {
        Ok(p) => {
            println!("HTML parsing, OK !");
            p
        },
        Err(e) => { panic!("An error occurred while using migrations files: {e:?}") }
    };

    let movie_repo = init_movie_repository(db);
    match movie_repo.save(movies).await {
        Ok(p) => {
            println!("Successfully saved movies !");
            p
        },
        Err(e) => { panic!("An error occurred while using migrations files: {e:?}") }
    };
}