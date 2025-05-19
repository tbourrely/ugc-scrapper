use chrono::NaiveDate;
use sqlx::PgPool;
use crate::database::models::Theater;
use crate::database::repositories::movie::init_movie_repository;
use crate::errors::Error;
use crate::features::scrapper::services::html_parser::HtmlParser;
use crate::features::scrapper::services::ugc::Ugc;
use crate::features::scrapper::utils::{dates, theaters};

pub async fn retrieve_movies_from_ugc (db: &PgPool) -> Result<(), Error> {
    let dates: Vec<NaiveDate> = dates::get_each_dates_of_current_week(None);
    let theaters: Vec<Theater> = theaters::get_lyon_theaters();

    let html_from_theaters = match Ugc::get_html_from_theaters_per_dates(theaters, dates).await {
        Ok(html) => html,
        Err(e) => return Err(Error::Reqwest(e))
    };

    println!("Successfully retrieved html from each theater on ugc web site");

    let movies = match HtmlParser::get_movies_from_html(&html_from_theaters) {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Scrapper(e))
    };

    println!("HTML parsing, OK !");

    let movie_repo = init_movie_repository(db);
    match movie_repo.save(movies).await {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Sqlx(e))
    };

    println!("Successfully saved movies !");

    Ok(())
}