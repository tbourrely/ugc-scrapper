use crate::database::{models::Movie, repositories::movie::MovieRepository};
use crate::features::discord::poll_domain::PollApiUpsertPayload;
use chrono::NaiveDate;
use sqlx::{Error, PgPool};

pub struct MoviesUseCases<'a> {
    movie_repository: MovieRepository<'a>,
}
impl MoviesUseCases<'_> {
    pub fn new(pool: &PgPool) -> MoviesUseCases<'_> {
        MoviesUseCases {
            movie_repository: MovieRepository { pool },
        }
    }

    pub fn generate_polls_for_movies(&self, movies: Vec<String>) -> PollApiUpsertPayload {
        let mut poll = PollApiUpsertPayload::new(
            String::from("30 18 * * 1"),
            String::from("Quel film ?"),
            movies,
        );
        poll.onetime = true;
        return poll;
    }

    pub async fn get_movies(
        &self,
        due_date: Vec<NaiveDate>,
        title_excluded: Vec<String>,
    ) -> Result<Vec<Movie>, Error> {
        let movies = self
            .movie_repository
            .retrieve_movies_for_specific_date(due_date, title_excluded)
            .await?;

        Ok(movies.values().cloned().collect())
    }
}
