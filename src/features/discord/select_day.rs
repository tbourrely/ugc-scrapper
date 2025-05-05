use std::env;
use sqlx::PgPool;
use crate::database::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::services::poll::{PollApiUpsertPayload, PollGeneratorApi};

pub async fn generate_poll_to_select_days (db: &PgPool) -> Result<(), Error> {
    let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");
    let url = String::from(discord_api_base_url) + "/polls";

    let poll: PollApiUpsertPayload = PollApiUpsertPayload::new(
        None,
        String::from("15 16 * * 0"),
        String::from("Quel jour ?"),
        vec![
            String::from("mardi"),
            String::from("mercredi"),
            String::from("jeudi"),
            String::from("vendredi"),
            String::from("samedi"),
            String::from("dimanche"),
            String::from("lundi")
        ]
    );

    let poll_payload = match PollGeneratorApi::initiate_poll_creation(poll, url).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Reqwest(e))
    };

    let generated_poll = poll_payload.transform();

    let poll_repository = init_poll_repository(db);
    match poll_repository.save(generated_poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e))
    };

    Ok(())
}