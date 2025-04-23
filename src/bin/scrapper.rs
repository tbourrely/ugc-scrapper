use std::env;
use std::process::ExitCode;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use ugc_scrapper::migrations::init_db;
use ugc_scrapper::features::scrapper::scrapper;

#[tokio::main]
async fn main() -> ExitCode {
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
        Err(e) => {
            println!("failed to connect to DATABASE_URL: {e:?}");
            return ExitCode::from(ExitCode::FAILURE);
        }
    };

    match init_db(&pool).await {
        Ok(p) => {
            println!("Migration Finished");
            p
        },
        Err(e) => {
            println!("An error occurred while using migrations files: {e:?}");
            return ExitCode::from(ExitCode::FAILURE);
        }
    };

    match scrapper::retrieve_movies_from_ugc(&pool).await {
        Ok(movies) => movies,
        Err(e) => {
            println!("Failed to retrieve movies: {e:?}");
            return ExitCode::from(ExitCode::FAILURE);
        }
    };

    ExitCode::from(ExitCode::SUCCESS)
}

