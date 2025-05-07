use std::process::ExitCode;
use dotenv::dotenv;
use ugc_scrapper::database::init_db::init_db;
use ugc_scrapper::features::scrapper::scrapper;

#[tokio::main]
async fn main() -> ExitCode {
    dotenv().ok();
    let pool = match init_db().await {
        Ok(p) => p,
        Err(e) => {
            println!("An error occurred while trying to init db: {e:?}");
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

