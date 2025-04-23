use std::collections::HashMap;
use sqlx::{Error, PgPool, Row};
use uuid::Uuid;
use crate::database::domain::{Movie, MoviesFromHtml, Theater};
use crate::features::scrapper::utils::theaters;

pub fn init_movie_repository(pool: &PgPool) -> MovieRepository {
    MovieRepository { pool: &pool }
}

pub struct MovieRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> MovieRepository<'a> {
    async fn resolve_theater_id (&self) -> Result<HashMap<Theater, Uuid>, Error> {
        let mut theaters_uuid: HashMap<Theater, Uuid> = HashMap::new();
        let lyon_theaters = theaters::get_lyon_theaters();

        let rows = sqlx::query(
            r#"SELECT id, ugc_identifier FROM theaters WHERE ugc_identifier = ANY($1)"#
        )
            .bind(&lyon_theaters[..])
            .fetch_all(self.pool)
            .await?;

        for row in rows {
            theaters_uuid.insert(row.get(1), row.get(0));
        }

        Ok(theaters_uuid)
    }

    pub async fn get_existing_movies_by_titles(&self, movie_titles: Vec<String>) -> Result<HashMap<String, Uuid>, Error> {
        let mut movie_uuid_by_title: HashMap<String, Uuid> = HashMap::new();

        let rows = sqlx::query(
            r#"SELECT id, title FROM movies WHERE title = ANY($1)"#
        )
            .bind(&movie_titles[..])
            .fetch_all(self.pool)
            .await?;

        for row in rows {
            movie_uuid_by_title.insert(row.get(1), row.get(0));
        }

        Ok(movie_uuid_by_title)
    }

    pub async fn save(&self, movies: MoviesFromHtml) -> Result<Vec<Movie>, Error> {
        let mut movies_in_db: Vec<Movie> = Vec::new();

        let theater_hash_map = self.resolve_theater_id().await?;

        let mut movie_uuid_by_title = self.get_existing_movies_by_titles(movies.keys().cloned().collect()).await?;

        let mut movie_uuids: Vec<Uuid> = Vec::new();
        let mut movie_titles: Vec<String> = Vec::new();
        let mut movie_grades: Vec<f32> = Vec::new();

        let mut screening_ids: Vec<Uuid> = Vec::new();
        let mut screening_movie_ids: Vec<Uuid> = Vec::new();
        let mut screening_theater_ids: Vec<Uuid> = Vec::new();
        let mut screening_due_dates: Vec<String> = Vec::new();
        let mut screening_hours: Vec<String> = Vec::new();

        for (title, movie) in movies {

            if !movie_uuid_by_title.contains_key(&title) {
                movie_uuids.push(movie.id);
                movie_titles.push(movie.title.clone());
                movie_grades.push(movie.grade);
            }

            for screening in &movie.screenings {
                screening_ids.push(screening.id);

                if movie_uuid_by_title.contains_key(&title) {
                    screening_movie_ids.push(*movie_uuid_by_title.get_mut(&title).unwrap());
                } else {
                    screening_movie_ids.push(movie.id);
                }

                screening_theater_ids.push(*theater_hash_map.get((&screening.theater).into()).unwrap());
                screening_due_dates.push(screening.due_date.to_string());
                screening_hours.push(serde_json::to_string(&screening.hours.hours).unwrap());
            }

            movies_in_db.push(movie);
        }

        sqlx::query(
            "
                INSERT INTO movies(id, title, grade)
                SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::real[])
            ",
        )
        .bind(&movie_uuids[..])
        .bind(&movie_titles[..])
        .bind(&movie_grades[..])
        .execute(self.pool)
        .await?;

        sqlx::query(
            "
                INSERT INTO screenings(id, movie_id, theater_id, screenings_time, due_date)
                SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::uuid[], $4::json[], $5::text[])
            "
        )
            .bind(&screening_ids[..])
            .bind(&screening_movie_ids[..])
            .bind(&screening_theater_ids[..])
            .bind(&screening_hours[..])
            .bind(&screening_due_dates[..])
            .execute(self.pool)
            .await?;

        Ok(movies_in_db)
    }
}
