use std::env;
use anyhow::Context;
use dotenv::dotenv;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use ugc_scrapper::api::ugc_movies;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = get_database_url();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("failed to connect to DATABASE_URL")?;

    let app = Router::new()
        .merge(ugc_movies::router())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await.context("failed to serve API")
}

fn get_database_url() -> String {
    dotenv().ok();

    let database_name = env::var("DATABASE_NAME").expect("Expected DATABASE_NAME in the environment");
    let database_username = env::var("DATABASE_USERNAME").expect("Expected DATABASE_USERNAME in the environment");
    let database_password = env::var("DATABASE_PASSWORD").expect("Expected DATABASE_PASSWORD in the environment");

    format!("postgres://{}:{}@db/{}", database_username, database_password, database_name)
}