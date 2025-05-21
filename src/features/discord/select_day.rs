use sqlx::PgPool;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::poll_domain::PollApiUpsertPayload;
use crate::features::discord::use_cases::poll_api_use_case::{PollApiUseCase};

pub async fn generate_poll_to_select_days (db: &PgPool) -> Result<(), Error> {
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

    let poll_api_use_case = PollApiUseCase::new();
    let poll_payload = match poll_api_use_case.initiate_poll_creation(poll).await {
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