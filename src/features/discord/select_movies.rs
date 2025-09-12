use crate::database::models::{Movie, PollType};
use crate::database::repositories::answer::init_answer_repository;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::use_cases::movies_use_cases::MoviesUseCases;
use crate::features::discord::use_cases::poll_api_use_case::PollApiUseCase;
use crate::utils::dates;
use chrono::{NaiveTime, Timelike};
use sqlx::PgPool;

// filters out movies that only have screenings before 19:00
fn filter_movies_based_on_screenings(movies: Vec<Movie>) -> Vec<Movie> {
    movies
        .into_iter()
        .filter(|movie| {
            for screening in &movie.screenings {
                for hour in &screening.hours {
                    if let Ok(parsed_time) = NaiveTime::parse_from_str(hour, "%H:%M")
                        && parsed_time.hour() >= 19
                    {
                        return true;
                    }
                }
            }
            false
        })
        .collect()
}

pub async fn generate_poll_to_select_movies(db: &PgPool) -> Result<(), Error> {
    // get poll distant id
    let poll_repository = init_poll_repository(db);
    let option_poll = match poll_repository.get_last_day_poll().await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e)),
    };

    if option_poll.is_none() {
        return Err(Error::Other("No poll were found".to_string()));
    }

    let poll = option_poll.unwrap();

    // get day(s) from poll
    let poll_api_use_case = PollApiUseCase::new();
    let desired_days = match poll_api_use_case
        .get_most_voted_answers_from_poll_answers(poll.distant_id)
        .await
    {
        Ok(most_voted_answer) => most_voted_answer,
        Err(e) => return Err(Error::Reqwest(e)),
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
        Err(e) => return Err(Error::Sqlx(e)),
    };

    println!("MOVIES SEEN : {:?}", movies_seen);

    // retrieve all movies titles which have screenings on specified dates without movies seen
    let movie_use_case = MoviesUseCases::new(db);
    let mut movies = match movie_use_case
        .get_movies(
            dates,
            movies_seen.iter().map(|m| m.content.clone()).collect(),
        )
        .await
    {
        Ok(movies) => movies,
        Err(e) => return Err(Error::Sqlx(e)),
    };

    movies = filter_movies_based_on_screenings(movies);

    let poll =
        movie_use_case.generate_polls_for_movies(movies.iter().map(|m| m.title.clone()).collect());

    if poll.answers.is_empty() {
        println!("Il n'y a pas de nouveau film à voir.");
        return Ok(());
    }
    println!("{:?}", poll);

    let poll_api_use_case = PollApiUseCase::new();
    let poll = match poll_api_use_case
        .initiate_poll_creation(poll, PollType::SelectMovie)
        .await
    {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Reqwest(e)),
    };

    let poll_repository = init_poll_repository(db);
    match poll_repository.save(poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e)),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use uuid::Uuid;

    use super::*;
    use crate::database::models::{Movie, Screening};

    #[test]
    fn test_filter_movies_based_on_screenings_returns_no_movies() {
        let movies = vec![Movie {
            id: Uuid::new_v4(),
            title: "Movie 1".to_string(),
            grade: 1.0,
            screenings: vec![Screening::new(
                None,
                1,
                NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                vec!["10:00".to_string(), "14:00".to_string()],
            )],
        }];
        let filtered = filter_movies_based_on_screenings(movies.clone());
        assert_eq!(0, filtered.len(), "All movies should be filtered out");
    }

    #[test]
    fn test_filter_movies_based_on_screenings_returns_all_movies() {
        let movies = vec![Movie {
            id: Uuid::new_v4(),
            title: "Movie 1".to_string(),
            grade: 1.0,
            screenings: vec![Screening::new(
                None,
                1,
                NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                vec!["10:00".to_string(), "19:00".to_string()],
            )],
        }];
        let filtered = filter_movies_based_on_screenings(movies.clone());
        assert_eq!(movies.len(), filtered.len(), "All movies should be kept");
    }

    #[test]
    fn test_filter_movies_based_on_screenings_returns_single_valid_movie() {
        let movies = vec![
            Movie {
                id: Uuid::new_v4(),
                title: "Movie 1".to_string(),
                grade: 1.0,
                screenings: vec![Screening::new(
                    None,
                    1,
                    NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                    vec!["10:00".to_string(), "18:59".to_string()],
                )],
            },
            Movie {
                id: Uuid::new_v4(),
                title: "Movie 2".to_string(),
                grade: 1.0,
                screenings: vec![Screening::new(
                    None,
                    1,
                    NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
                    vec!["10:00".to_string(), "19:00".to_string()],
                )],
            },
        ];
        let filtered = filter_movies_based_on_screenings(movies.clone());
        assert_eq!(1, filtered.len(), "One movie should be kept");
    }
}
