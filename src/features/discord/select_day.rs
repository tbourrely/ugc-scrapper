use sqlx::PgPool;
use crate::database::models::PollType;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::poll_domain::PollApiUpsertPayload;
use crate::features::discord::use_cases::poll_api_use_case::{PollApiUseCase};

pub async fn generate_poll_to_select_days (db: &PgPool) -> Result<(), Error> {
    let poll: PollApiUpsertPayload = PollApiUpsertPayload::new(
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

    let poll_api_use_case = PollApiUseCase::new();
    let poll = match poll_api_use_case.initiate_poll_creation(poll, PollType::SelectDay).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Reqwest(e))
    };

    let poll_repository = init_poll_repository(db);
    match poll_repository.save(poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e))
    };

    Ok(())
}