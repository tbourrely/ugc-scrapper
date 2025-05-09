use std::env;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::errors::Error;

pub async fn init_db() -> Result<Pool<Postgres>, Error> {
    let database = env::var("DATABASE_URL").expect("Expected DATABASE in the environment");

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database)
        .await {
        Ok(pool) => pool,
        Err(e) => return Err(Error::Sqlx(e))
    };

    match crate::database::migrations::run(&pool).await {
        Ok(p) => p,
        Err(e) => return Err(Error::SqlxMigration(e))
    };

    Ok(pool)
}