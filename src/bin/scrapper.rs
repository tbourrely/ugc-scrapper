use dotenv::dotenv;
use env_logger;
use log::error;
use std::process::ExitCode;
use ugc_scrapper::database::init_db::init_db;
use ugc_scrapper::features::scrapper::scrapper;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    dotenv().ok();
    let pool = match init_db().await {
        Ok(p) => p,
        Err(e) => {
            error!("An error occurred while trying to init db: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    match scrapper::retrieve_movies_from_ugc(&pool).await {
        Ok(movies) => movies,
        Err(e) => {
            error!("Failed to retrieve movies: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
