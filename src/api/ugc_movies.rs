use axum::{extract::{Json}, Extension};
use crate::scrapper::Scrapper;
use axum::{routing::get, Router};
use sqlx::{PgPool};
use crate::api::dto::{JsonFromRequest};
use crate::api::errors::Error;
use crate::database::domain::{Movie};
use crate::database::movie::MoviesRepository;
use crate::services::discord::Discord;
use crate::services::ugc::Ugc;

pub fn router() -> Router {
    Router::new()
        .route("/retrieve-ugc-movies", get(retrieve_movies_from_ugc))
}

pub async fn retrieve_movies_from_ugc(
    db: Extension<PgPool>,
    Json(payload): Json<JsonFromRequest>,
) -> Result<Json<Vec<Movie>>, Error> {
    let transformed_payload = match payload.transform().await {
        Ok(transformed_payload) => transformed_payload,
        Err(e) => return Err(Error::Reqwest(e)),
    };

    let html_per_theaters_per_date = match Ugc::get_html_from_theaters_per_dates(
        transformed_payload.theaters,
        transformed_payload.dates
    ).await {
        Ok(html) => html,
        Err(e) => return Err(Error::Reqwest(e))
    };

    let movies = match Scrapper::get_movies_from_html(html_per_theaters_per_date) {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Scrapper(e))
    };

    let stored_movies = match MoviesRepository::save(db, movies).await {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Sqlx(e))
    };

    let discord = Discord::generate_poll_with_movies(stored_movies.clone()).await;
    if discord.is_err() {
        return Err(Error::Reqwest(discord.err().unwrap()));
    }

    Ok(Json(stored_movies))
}