use sqlx::migrate::MigrateError;
use sqlx::{Pool, Postgres};

pub async fn run(db: &Pool<Postgres>) -> Result<(), MigrateError> {
    Ok(sqlx::migrate!().run(db).await?)
}