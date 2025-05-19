use std::env;
use sqlx::PgPool;
use crate::errors::Error;

pub async fn _generate_poll_to_select_movies (_db: &PgPool) -> Result<(), Error> {
    let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");
    let _url = String::from(discord_api_base_url) + "/polls";

    // get day(s) from poll

    // retrieve movies not seen since 2 month

    // generate poll for movies
    // ⚠ poll in discord are limited to just 10 possible answers.
    // ⚠ each answer are limit to just 55 characters

    /*let poll_payload = match PollApiUseCase::initiate_poll_creation(poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Reqwest(e))
    };*/

    /*let generated_poll = poll_payload.transform();

    let poll_repository = init_poll_repository(db);
    match poll_repository.save(generated_poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e))
    };*/

    Ok(())
}