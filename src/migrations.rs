use sqlx::migrate::MigrateError;
use sqlx::{Pool, Postgres};

pub async fn init_db(db: &Pool<Postgres>) -> Result<(), MigrateError> {
    Ok(sqlx::migrate!().run(db).await?)
}