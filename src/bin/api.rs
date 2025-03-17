use std::env;
use dotenv::dotenv;
use ugc_scrapper::migrations::init_db;
use axum::{
    routing::get,
    Router,
};
use ugc_scrapper::api::handlers::{
    retrieve_screenings_from_db,
    retrieve_movies_from_ugc
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database = env::var("DATABASE").expect("Expected DATABASE in the environment");
    init_db(&database).ok();

    let app = Router::new()
        .route(
            "/retrieve-ugc-movies",
            get(retrieve_movies_from_ugc)
        )
        .route(
            "/current-screenings",
            get(retrieve_screenings_from_db)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}