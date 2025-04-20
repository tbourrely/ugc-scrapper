use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use scraper::error::SelectorErrorKind;
use serde::{Serialize};
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A SQLx call returned an error.
    ///
    /// The exact error contents are not reported to the user in order to avoid leaking
    /// information about databse internals.
    #[error("an internal database error occurred")]
    Sqlx(#[from] sqlx::Error),

    #[error("An error occurred while trying to retrieve movies from UGC")]
    Reqwest(#[from] reqwest::Error),

    #[error("An error occurred while parsing UGC page")]
    Scrapper(#[from] SelectorErrorKind<'static>),

    /// Similarly, we don't want to report random `anyhow` errors to the user.
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),

    #[error("{0}")]
    UnprocessableEntity(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[serde_as]
        #[skip_serializing_none]
        #[derive(Serialize)]
        struct ErrorResponse<'a> {
            // Serialize the `Display` output as the error message
            #[serde_as(as = "DisplayFromStr")]
            message: &'a Error,

            errors: Option<&'a ValidationErrors>,
        }

        let errors = match &self {
            Error::InvalidEntity(errors) => Some(errors),
            _ => None,
        };


        // Normally you wouldn't just print this, but it's useful for debugging without
        // using a logging framework.
        println!("API error: {self:?}");

        (
            self.status_code(),
            Json(ErrorResponse {
                message: &self,
                errors,
            }),
        )
            .into_response()
    }
}

impl Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;

        match self {
            Sqlx(_) | Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidEntity(_) | UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Reqwest(_) | Scrapper(_) => StatusCode::BAD_GATEWAY,
        }
    }
}