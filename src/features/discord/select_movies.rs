use sqlx::PgPool;
use crate::database::models::PollType;
use crate::database::repositories::answer::init_answer_repository;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::use_cases::movies_use_cases::MoviesUseCases;
use crate::features::discord::use_cases::poll_api_use_case::PollApiUseCase;
use crate::utils::dates;

pub async fn generate_poll_to_select_movies (db: &PgPool) -> Result<(), Error> {
    // get poll distant id
    let poll_repository = init_poll_repository(db);
    let option_poll = match poll_repository.get_last_day_poll().await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e))
    };

    if option_poll.is_none() {
        return Err(Error::Other("No poll were found".to_string()));
    }

    let poll = option_poll.unwrap();

    // get day(s) from poll
    let poll_api_use_case = PollApiUseCase::new();
    let desired_days = match poll_api_use_case.get_most_voted_answers_from_poll_answers(poll.distant_id).await {
        Ok(most_voted_answer) => most_voted_answer,
        Err(e) => return Err(Error::Reqwest(e))
    };

    println!("{:?}", desired_days);

    // transform days to NaiveDate
    let dates;
    if desired_days.is_empty() {
        dates = dates::get_each_dates_of_current_week(None);
    } else {
        dates = dates::get_date_from_day_name(desired_days)
            .unwrap_or_else(|_| dates::get_each_dates_of_current_week(None));
    }

    println!("{:?}", dates);

    // retrieve movies seen since in the last two month
    let answer_repository = init_answer_repository(db);
    let movies_seen = match answer_repository.get_answers_since_two_month().await {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Sqlx(e))
    };

    println!("MOVIES SEEN : {:?}", movies_seen);

    // retrieve all movies titles which have screenings on specified dates without movies seen
    let movie_use_case = MoviesUseCases::new(db);
    let movies = match movie_use_case.get_movie_titles(dates, movies_seen.iter().map(|m| m.content.clone()).collect()).await {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Sqlx(e))
    };

    let poll = movie_use_case.generate_polls_for_movies(movies);

    if poll.answers.len() == 0 {
        println!("Il n'y a pas de nouveau film à voir.");
        return Ok(())
    }
    println!("{:?}", poll);

    let poll_api_use_case = PollApiUseCase::new();
    let poll = match poll_api_use_case.initiate_poll_creation(poll, PollType::SelectMovie).await {
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