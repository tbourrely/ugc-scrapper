use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{Error, PgPool, Row};
use crate::database::models::{Poll, PollType};

pub fn init_poll_repository(pool: &PgPool) -> PollRepository {
    PollRepository { pool: &pool }
}

pub struct PollRepository<'a> {
    pub pool: &'a PgPool,
}

impl<'a> PollRepository<'a> {
    pub async fn save(&self, poll: Poll) -> Result<Poll, Error> {

        sqlx::query( "
            INSERT INTO polls (id, distant_id, type)
            VALUES ($1, $2, $3)
        ")
            .bind(poll.id)
            .bind(poll.distant_id)
            .bind(poll.get_poll_type_number())
            .execute(self.pool)
            .await?;

        Ok(poll)
    }

    pub async fn get_last_day_poll(&self) -> Result<Poll, Error> {
        let row = sqlx::query("
            SELECT * FROM polls
            WHERE type = $1
        ")
        .bind(PollType::SelectDay as i16)
        .fetch_one(self.pool)
        .await?;

        Ok(Poll {
            id: row.get::<uuid::Uuid, usize>(0),
            distant_id: Some(row.get::<uuid::Uuid, usize>(1)),
            poll_type: PollType::SelectDay,
            created_at: Some(NaiveDate::from(row.get::<NaiveDateTime, usize>(3))),
        })
    }
}