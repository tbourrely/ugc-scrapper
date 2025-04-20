use std::env;
use axum::http::header::{ACCEPT};
use axum::http::{HeaderMap, HeaderValue};
use reqwest::Error;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use crate::database::domain::Movie;

#[derive(Debug, Deserialize)]
pub struct PollAnswer {
   pub answer: String,
}

#[derive(Debug, Serialize)]
struct Poll {
    cron: String,
    question: String,
    answers: Vec<String>,
    multiselect: bool,
    guild: String,
    channel: String,
    duration: i32
}

pub struct Discord {}
impl Discord {
    pub async fn get_answers_from_last_poll() -> Result<Vec<PollAnswer>, Error> {
        let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");
        let url = String::from(discord_api_base_url) + "/poll_answers/most_recent_poll";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();
        let response = match client.get(&url).headers(headers).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let answers = match response.json::<Vec<PollAnswer>>().await {
            Ok(answers) => answers,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        Ok(answers)
    }

    pub async fn generate_poll_with_movies(movies: Vec<Movie>) -> Result<(), Error> {
        let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");
        let url = String::from(discord_api_base_url) + "/polls";

        let poll = Discord::create_poll_for_movies(movies);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();

        match client.post(&url).headers(headers).json(&poll).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        Ok(())
    }

    fn create_poll_for_movies(movies: Vec<Movie>) -> Poll {
        Poll {
            cron: String::from("30 18 * * 1"),
            question: String::from("Quel film ?"),
            answers: movies.iter().map(|m| m.title.clone()).collect(),
            multiselect: true,
            guild:  String::from("le club des cinephiles"),
            channel: String::from("cinema"),
            duration: 86400
        }
    }
}