use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Error, PgPool, Row};
use crate::database::models::{MoviesSeen};

pub fn init_answer_repository(pool: &PgPool) -> AnswerRepository {
    AnswerRepository { pool: &pool }
}

pub struct AnswerRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> AnswerRepository<'a> {
    pub async fn save(&self, answer: MoviesSeen) -> Result<MoviesSeen, Error> {

        sqlx::query( "
            INSERT INTO movies_seen (id, distant_id, created_at)
            VALUES ($1, $2)
        ")
            .bind(answer.id)
            .bind(answer.content.clone())
            .execute(self.pool)
            .await?;

        Ok(answer)
    }

    pub async fn get_answers_since_two_month(&self) -> Result<Vec<MoviesSeen>, Error> {
        let row = sqlx::query("
            SELECT * FROM answers
            WHERE created_at >= date_trunc('month',current_date) - INTERVAL '2' MONTH
        ")
        .fetch_all(self.pool)
        .await?;

        let mut answers = Vec::new();
        for answer in row {
            answers.push(
                MoviesSeen::new(
                    Some(answer.get::<uuid::Uuid, usize>(0)),
                    answer.get::<String, usize>(1),
                    Some(NaiveDate::from(answer.get::<NaiveDateTime, usize>(2)))
                )
            )
        }

        Ok(answers)
    }
}