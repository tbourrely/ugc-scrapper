use crate::database::models::MoviesSeen;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Error, PgPool, Row};

pub fn init_answer_repository(pool: &PgPool) -> AnswerRepository<'_> {
    AnswerRepository { pool }
}

pub struct AnswerRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> AnswerRepository<'a> {
    pub async fn get_answers_since_two_month(&self) -> Result<Vec<MoviesSeen>, Error> {
        let row = sqlx::query(
            "
            SELECT * FROM answers
            WHERE created_at >= date_trunc('month',current_date) - INTERVAL '2' MONTH
        ",
        )
        .fetch_all(self.pool)
        .await?;

        let mut answers = Vec::new();
        for answer in row {
            answers.push(MoviesSeen::new(
                Some(answer.get::<uuid::Uuid, usize>(0)),
                answer.get::<String, usize>(1),
                Some(NaiveDate::from(answer.get::<NaiveDateTime, usize>(2))),
            ))
        }

        Ok(answers)
    }
}
