use axum::{
    extract::{Json},
    http::StatusCode,
};
use crate::http_agent::HttpAgent;
use crate::scrapper::Scrapper;
use serde::{Deserialize, Serialize};
use crate::repository::Repository;

#[derive(Deserialize, Serialize, Debug)]
pub struct UgcFilterPayload {
    pub theaters: Option<Vec<i8>>,
    pub dates: Option<Vec<String>>
}

pub async fn retrieve_movies_from_ugc(
    Json(payload): Json<UgcFilterPayload>,
) -> Result<Json<String>, StatusCode> {
    println!("payload : {:?}", payload);

    let dates = HttpAgent::verify_or_set_default_dates(payload.dates).unwrap();
    let theaters = HttpAgent::verify_or_set_default_theaters(payload.theaters).unwrap();

    let pages_per_theaters_per_date = HttpAgent::get_theaters_html_pages_by_dates(theaters, dates).await;

    let screenings = Scrapper::scrap_screenings_from_ugc_html_page(pages_per_theaters_per_date);

    Repository::save(screenings);

    let t: String = String::from("retrieve_movies_from_ugc");
    Ok(Json(t))
}

pub async fn retrieve_screenings_from_db() -> Result<Json<String>, StatusCode> {
    let t: String = String::from("retrieve_screenings_from_db");
    Ok(Json(t))
}