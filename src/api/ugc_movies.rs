use axum::{extract::{Json}, http::StatusCode, Extension};
use crate::http_agent::HttpAgent;
use crate::scrapper::Scrapper;
use axum::{routing::get, Router};
use sqlx::PgPool;
use crate::api::dto::{JsonFromRequest};
use crate::database::repository::Repository;

pub fn router() -> Router {
    Router::new().route("/retrieve-ugc-movies", get(retrieve_movies_from_ugc))
}

pub async fn retrieve_movies_from_ugc(
    db: Extension<PgPool>,
    Json(payload): Json<JsonFromRequest>,
) -> Result<Json<String>, StatusCode> {
    println!("payload : {:?}", payload);
    let transformed_payload = payload.transform();

    let html_per_theaters_per_date = HttpAgent::get_html_from_theaters_per_dates(
        transformed_payload.theaters,
        transformed_payload.dates
    ).await;

    let screenings = Scrapper::get_screenings_from_html(html_per_theaters_per_date);

    println!("{:?}", screenings);

    Repository::save(db, screenings).await;

    let t: String = String::from("retrieve_movies_from_ugc");
    Ok(Json(t))
}