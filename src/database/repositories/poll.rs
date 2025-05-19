use sqlx::{Error, PgPool};
use crate::database::models::{Poll};

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
}