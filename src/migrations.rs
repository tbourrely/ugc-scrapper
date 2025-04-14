use sqlx::migrate::MigrateError;
use sqlx::PgPool;

pub async fn init_db(db: PgPool) -> Result<(), MigrateError> {
    Ok(sqlx::migrate!().run(&db).await?)
}