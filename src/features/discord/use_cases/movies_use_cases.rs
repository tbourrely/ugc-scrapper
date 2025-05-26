use chrono::NaiveDate;
use sqlx::{Error, PgPool};
use crate::database::repositories::movie::{init_movie_repository, MovieRepository};
use crate::features::discord::poll_domain::PollApiUpsertPayload;

pub struct MoviesUseCases<'a> {
    movie_repository: MovieRepository<'a>
}
impl MoviesUseCases<'_> {
    pub fn new(pool: &PgPool) -> MoviesUseCases {
        MoviesUseCases {
            movie_repository: init_movie_repository(pool),
        }
    }

    pub fn generate_polls_for_movies(&self, movies: Vec<String>) -> PollApiUpsertPayload {
        PollApiUpsertPayload::new(
            String::from("30 18 * * 1"),
            String::from("Quel film ?"),
            movies
        )
    }

    pub async fn get_movie_titles(&self, due_date: Vec<NaiveDate>, title_excluded: Vec<String>) -> Result<Vec<String>, Error> {
        let movies = self.movie_repository.retrieve_movies_for_specific_date(due_date, title_excluded).await?;

        Ok(movies.values().map(|m| m.title.clone()).collect())
    }
}