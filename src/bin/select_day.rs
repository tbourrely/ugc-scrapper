use std::process::ExitCode;
use dotenv::dotenv;
use ugc_scrapper::database::init_db::init_db;
use ugc_scrapper::features::discord::select_day;

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

    match select_day::generate_poll_to_select_days(&pool).await {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to generate poll to select days: {e:?}");
            return ExitCode::from(ExitCode::FAILURE);
        }
    };

    ExitCode::from(ExitCode::SUCCESS)
}