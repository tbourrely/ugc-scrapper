use std::{env, process};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use ugc_scrapper::migrations::init_db;
use ugc_scrapper::features::scrapper::scrapper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database = env::var("DATABASE_URL").expect("Expected DATABASE in the environment");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await {
        Ok(p) => {
            println!("Db connection established");
            p
        },
        Err(e) => { panic!("failed to connect to DATABASE_URL: {e:?}") }
    };

    match init_db(&pool).await {
        Ok(p) => {
            println!("Migration Finished");
            p
        },
        Err(e) => { panic!("An error occurred while using migrations files: {e:?}") }
    };

    scrapper::retrieve_movies_from_ugc(&pool).await;

    process::exit(0x0100);
}

