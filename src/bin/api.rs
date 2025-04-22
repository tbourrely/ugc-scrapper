use std::env;
use anyhow::Context;
use dotenv::dotenv;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use ugc_scrapper::api::ugc_movies;
use ugc_scrapper::migrations::init_db;
use ugc_scrapper::utils::scheduler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database = env::var("DATABASE_URL").expect("Expected DATABASE in the environment");

    let mut pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await
        .context("failed to connect to DATABASE_URL")?;

    init_db(&pool).await?;

    let app = Router::new()
        .merge(ugc_movies::router())
        .layer(Extension(&pool));

    let port_api = env::var("PORT_API").expect("Expected PORT_API in the environment");
    let host = "0.0.0.0:".to_owned() + port_api.as_str();
    let listener = tokio::net::TcpListener::bind(host).await?;

    scheduler::start_scheduler(&pool);

    axum::serve(listener, app).await.context("failed to serve API")
}