use dotenv::dotenv;
use env_logger;
use log::error;
use std::process::ExitCode;
use ugc_scrapper::database::init_db::init_db;
use ugc_scrapper::features::discord::select_day;

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

    match select_day::generate_poll_to_select_days(&pool).await {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to generate poll to select days: {e:?}");
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
