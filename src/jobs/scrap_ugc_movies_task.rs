use chrono::NaiveDate;
use sqlx::{Pool, Postgres};
use crate::database::domain::Theater;
use crate::database::movie::{init_movie_repository};
use crate::scrapper::Scrapper;
use crate::services::ugc::Ugc;
use crate::utils::{dates, theaters};

pub async fn test () -> () {
    println!("{}", "hello world");
    ()
}
pub async fn retrieve_movies_from_ugc(db: &Pool<Postgres>) -> () {
    let dates: Vec<NaiveDate> = dates::get_each_dates_of_current_week(None);
    let theaters: Vec<Theater> = theaters::get_lyon_theaters();

    let html_per_theaters_per_dates = Ugc::get_html_from_theaters_per_dates(theaters, dates).await;
    if html_per_theaters_per_dates.is_err() {
        log::error!("{:?}", &html_per_theaters_per_dates.unwrap_err());
        return ();
    }

    let movies = Scrapper::get_movies_from_html(&html_per_theaters_per_dates.ok().unwrap());
    if movies.is_err() {
        log::error!("{:?}", movies.clone().unwrap_err());
        return ();
    }

    let movie_repo = init_movie_repository(&db);
    let result = movie_repo.save(movies.unwrap()).await;
    if result.is_err() {
        log::error!("{:?}", result.unwrap_err());
        return ();
    }

    ()
}