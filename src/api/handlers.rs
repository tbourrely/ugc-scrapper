use axum::{
    Json,
    http::StatusCode,
};

pub struct UgcFilterPayload {
    pub theaters: Vec<u8>,
    pub dates: Vec<String>,
}

pub async fn retrieve_movies_from_ugc(
    Json(payload): Json<UgcFilterPayload>,
) -> Result<Json<String>, StatusCode> {
    println!("{}", serde_json::to_string_pretty(&payload).unwrap());
    let t: String = String::from("retrieve_movies_from_ugc");
    Ok(Json(t))
}

pub async fn retrieve_screenings_from_db() -> Result<Json<String>, StatusCode> {
    let t: String = String::from("retrieve_screenings_from_db");
    Ok(Json(t))
}