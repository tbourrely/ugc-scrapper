use axum::Extension;
use sqlx::PgPool;
use crate::database::domain::Screening;

pub struct Repository {}
impl Repository {
    pub async fn save(_db: Extension<PgPool>, _screening: Vec<Screening>) {
        /*sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO movies (id, title, grade, synopsis  )
                VALUES (1, 'A', 'X'), (2, 'B', 'Y'), (3, 'C', 'Z')
                ON CONFLICT (id) DO UPDATE
                  SET column_1 = excluded.column_1,
                      column_2 = excluded.column_2;
            "#,
            username,
            password_hash
        )
        .execute(&*db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
                Error::Conflict("username taken".into())
            }
            _ => e.into(),
        })?;*/
    }
    pub fn _get(_id: uuid::Uuid) {}
    pub fn _delete(_screening: Screening) {}
}
