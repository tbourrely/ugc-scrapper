use axum::{
    extract::{Json},
    http::StatusCode,
};
use crate::services::ugc::Ugc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UgcFilterPayload {
    pub theaters: Vec<i8>,
    pub dates: Vec<String>,
}

/*fn verify_theaters(theaters: Vec<Option<i8>>) -> Result<Vec<i8>, (StatusCode, String)> {
    match theaters {
        Ok(theaters) => {
            theaters
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            Err((
                StatusCode::BAD_REQUEST,
                "Missing `Content-Type: application/json` header".to_string(),
            ))
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
            Err((
                StatusCode::BAD_REQUEST,
                "Couldn't deserialize the body into the target type".to_string(),
            ))
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
            Err((
                StatusCode::BAD_REQUEST,
                "Syntax error in the Json body".to_string(),
            ))
        }
        Err(JsonRejection::BytesRejection(_)) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to buffer request body".to_string(),
            ))
        }
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            ))
        }
    }
}

fn verify_dates(dates: Vec<Option<String>>) -> Result<Vec<u8>, (StatusCode, String)> {
    match dates {
        Ok(dates) => {
            dates
        }
        Err(JsonRejection::MissingJsonContentType(_)) => {
            Err((
                StatusCode::BAD_REQUEST,
                "Missing `Content-Type: application/json` header".to_string(),
            ))
        }
        Err(JsonRejection::JsonDataError(_)) => {
            // Couldn't deserialize the body into the target type
            Err((
                StatusCode::BAD_REQUEST,
                "Couldn't deserialize the body into the target type".to_string(),
            ))
        }
        Err(JsonRejection::JsonSyntaxError(_)) => {
            // Syntax error in the body
            Err((
                StatusCode::BAD_REQUEST,
                "Syntax error in the Json body".to_string(),
            ))
        }
        Err(JsonRejection::BytesRejection(_)) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to buffer request body".to_string(),
            ))
        }
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown error".to_string(),
            ))
        }
    }
}*/

pub async fn retrieve_movies_from_ugc(
    Json(payload): Json<UgcFilterPayload>,
) -> Result<Json<String>, StatusCode> {
    println!("payload : {:?}", payload);

    // let theaters = verify_theaters(payload.theaters).unwrap();
    // let dates = verify_dates(payload.dates).unwrap();

    let _t = Ugc::get_theater_movies(payload.theaters, payload.dates);
    let t: String = String::from("retrieve_movies_from_ugc");
    Ok(Json(t))
}

pub async fn retrieve_screenings_from_db() -> Result<Json<String>, StatusCode> {
    let t: String = String::from("retrieve_screenings_from_db");
    Ok(Json(t))
}