use axum::{
    extract::{Json},
    http::StatusCode,
};
use crate::http_agent::HttpAgent;
use crate::scrapper::Scrapper;
use serde::{Deserialize, Serialize};
// use crate::repository::Repository;

#[derive(Deserialize, Serialize, Debug)]
pub struct UgcFilterPayload {
    pub theaters: Option<Vec<i8>>,
    pub dates: Option<Vec<String>>
}

pub async fn retrieve_movies_from_ugc(
    Json(payload): Json<UgcFilterPayload>,
) -> Result<Json<String>, StatusCode> {
    println!("payload : {:?}", payload);

    let dates = match HttpAgent::verify_or_set_default_dates(payload.dates) {
        Ok(d) => d,
        Err(e) => {
            println!("{:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let theaters = match HttpAgent::verify_or_set_default_theaters(payload.theaters) {
        Ok(d) => d,
        Err(e) => {
            println!("{:?}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    println!("theaters : {:?}", theaters);
    println!("theaters : {:?}", dates);

    let html_per_theaters_per_date = HttpAgent::get_html_from_theaters_per_dates(theaters, dates).await;

    let screenings = Scrapper::get_screenings_from_html(html_per_theaters_per_date);

    println!("{:?}", screenings);

    // Repository::save(screenings);

    let t: String = String::from("retrieve_movies_from_ugc");
    Ok(Json(t))
}

pub async fn retrieve_screenings_from_db() -> Result<Json<String>, StatusCode> {
    let t: String = String::from("retrieve_screenings_from_db");
    Ok(Json(t))
}