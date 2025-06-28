use scraper::error::SelectorErrorKind;
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("an error occurred while running migrations")]
    SqlxMigration(#[from] sqlx::migrate::MigrateError),

    #[error("An error occurred while sending request to another api")]
    Reqwest(#[from] reqwest::Error),

    #[error("An error occurred while parsing UGC page")]
    Scrapper(#[from] SelectorErrorKind<'static>),

    #[error("An error occurred")]
    Other(String),
}

impl Error {}