use sqlx::PgPool;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::use_cases::poll_api_use_case::PollApiUseCase;
use crate::utils::dates;

pub async fn generate_poll_to_select_movies (db: &PgPool) -> Result<(), Error> {
    // get poll distant id
    let poll_repository = init_poll_repository(db);
    let poll = match poll_repository.get_last_day_poll().await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e))
    };

    // get day(s) from poll
    let poll_api_use_case = PollApiUseCase::new();
    let voted_days = match poll_api_use_case.get_days_from_poll_answers(poll.distant_id.unwrap()).await {
        Ok(most_voted_answer) => most_voted_answer,
        Err(e) => return Err(Error::Reqwest(e))
    };

    println!("{:?}", voted_days);

    let dates;
    if voted_days.is_empty() {
        dates = dates::get_each_dates_of_current_week(None);
    } else {
        dates = dates::get_date_from_day_name(voted_days)
            .unwrap_or_else(|_| dates::get_each_dates_of_current_week(None));
    }
    println!("{:?}", dates);

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