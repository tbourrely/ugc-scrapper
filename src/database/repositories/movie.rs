use crate::database::models::{Movie, Screening, Theater};
use crate::utils::theaters;
use chrono::NaiveDate;
use sqlx::types::Json;
use sqlx::{Error, PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

pub fn init_movie_repository(pool: &PgPool) -> MovieRepository<'_> {
    MovieRepository { pool: &pool }
}

pub struct MovieRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> MovieRepository<'a> {
    async fn resolve_theater_id(&self) -> Result<HashMap<Theater, Uuid>, Error> {
        let mut theaters_uuid: HashMap<Theater, Uuid> = HashMap::new();
        let lyon_theaters = theaters::get_lyon_theaters();

        let rows = sqlx::query(
            r#"SELECT id, ugc_identifier FROM theaters WHERE ugc_identifier = ANY($1)"#,
        )
        .bind(&lyon_theaters[..])
        .fetch_all(self.pool)
        .await?;

        for row in rows {
            theaters_uuid.insert(row.get(1), row.get(0));
        }

        Ok(theaters_uuid)
    }

    pub async fn get_existing_movies_by_titles(
        &self,
        movie_titles: Vec<String>,
    ) -> Result<HashMap<String, Uuid>, Error> {
        let mut movie_uuid_by_title: HashMap<String, Uuid> = HashMap::new();

        let rows = sqlx::query(r#"SELECT id, title FROM movies WHERE title = ANY($1)"#)
            .bind(&movie_titles[..])
            .fetch_all(self.pool)
            .await?;

        for row in rows {
            movie_uuid_by_title.insert(row.get(1), row.get(0));
        }

        Ok(movie_uuid_by_title)
    }

    pub async fn save(&self, movies: Vec<Movie>) -> Result<Vec<Movie>, Error> {
        let theater_hash_map = self.resolve_theater_id().await?;

        let movie_titles: Vec<String> = movies.iter().map(|m| m.title.clone()).collect();
        let movie_uuid_by_title = self.get_existing_movies_by_titles(movie_titles).await?;

        let mut movie_uuids: Vec<Uuid> = Vec::new();
        let mut movie_titles: Vec<String> = Vec::new();
        let mut movie_grades: Vec<f32> = Vec::new();

        let mut screening_ids: Vec<Uuid> = Vec::new();
        let mut screening_movie_ids: Vec<&Uuid> = Vec::new();
        let mut screening_theater_ids: Vec<Uuid> = Vec::new();
        let mut screening_due_dates: Vec<NaiveDate> = Vec::new();
        let mut screening_hours: Vec<String> = Vec::new();

        for movie in &movies {
            let cloned_movie = movie.clone();

            if !movie_uuid_by_title.contains_key(&cloned_movie.title) {
                movie_uuids.push(movie.id);
                movie_titles.push(cloned_movie.title.clone());
                movie_grades.push(movie.grade);
            }

            for screening in movie.screenings.clone() {
                screening_ids.push(screening.id);

                if movie_uuid_by_title.contains_key(&cloned_movie.title) {
                    screening_movie_ids.push(movie_uuid_by_title.get(&movie.title).unwrap());
                } else {
                    screening_movie_ids.push(&movie.id);
                }

                screening_theater_ids
                    .push(*theater_hash_map.get((&screening.theater).into()).unwrap());
                screening_due_dates.push(screening.due_date);
                screening_hours.push(serde_json::to_string(&screening.hours).unwrap());
            }
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
                SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::uuid[], $4::json[], $5::date[])
            ",
        )
        .bind(&screening_ids[..])
        .bind(&screening_movie_ids[..])
        .bind(&screening_theater_ids[..])
        .bind(&screening_hours[..])
        .bind(&screening_due_dates[..])
        .execute(self.pool)
        .await?;

        Ok(movies)
    }

    pub async fn retrieve_movies_for_specific_date(
        &self,
        due_date: Vec<NaiveDate>,
        title_excluded: Vec<String>,
    ) -> Result<HashMap<String, Movie>, Error> {
        let rows = sqlx::query(
            "
                SELECT
                    m.id,
                    m.title,
                    m.grade,
                    s.id,
                    s.movie_id,
                    t.ugc_identifier,
                    s.screenings_time,
                    s.due_date
                FROM movies m
                INNER JOIN screenings s ON s.movie_id = m.id AND s.due_date = ANY ($1)
                LEFT JOIN theaters t ON t.id = s.theater_id
                WHERE title != ALL($2)
            ",
        )
        .bind(&due_date[..])
        .bind(&title_excluded[..])
        .fetch_all(self.pool)
        .await?;

        let mut movies: HashMap<String, Movie> = HashMap::new();
        for row in rows {
            let movie_title = row.get::<String, usize>(1);

            if !movies.contains_key(&movie_title) {
                let movie = Movie::new(
                    Some(row.get::<Uuid, usize>(0)),
                    row.get::<String, usize>(1),
                    row.get::<f32, usize>(2),
                );
                movies.insert(row.get::<String, usize>(1), movie);
            }

            let movie = movies.get_mut(&movie_title).unwrap();
            let hours: Vec<String> = row.get::<Json<Vec<String>>, usize>(6).to_vec();
            movie.screenings.push(Screening::new(
                Some(row.get::<Uuid, usize>(3)),
                row.get::<Theater, usize>(5),
                row.get::<NaiveDate, usize>(7),
                hours,
            ));
        }

        Ok(movies)
    }
}
