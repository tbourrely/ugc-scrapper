use std::env;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::domain::{Movie, Poll, PollType};

#[derive(Debug, Deserialize)]
pub struct PollAnswer {
   pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct PollApiUpsertPayload {
    pub id: Option<Uuid>,
    cron: String,
    question: String,
    answers: Vec<String>,
    multiselect: bool,
    guild: String,
    channel: String,
    duration: i32
}

impl PollApiUpsertPayload {
    pub fn new(id: Option<Uuid>, cron: String, question: String, answers: Vec<String>) -> Self {
        PollApiUpsertPayload {
            id,
            cron,
            question,
            answers,
            multiselect: true,
            guild:  DISCORD_GUILD,
            channel: DISCORD_CHANNEL,
            duration: 86400
        }
    }

    pub fn transform(mut self) -> Poll {
        Poll::new(self.id, PollType::SelectDay, None)
    }
}

const DISCORD_GUILD: String = String::from("le club des cinephiles");
const DISCORD_CHANNEL: String = String::from("cinema");

pub struct PollGeneratorApi {}
impl PollGeneratorApi {
    pub async fn initiate_poll_creation<T>(poll: T, url: String) -> Result<T, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();

        let result = match client.post(&url).headers(headers).json(&poll).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        match result.json().await {
            Ok(poll) => Ok(poll),
            Err(reqwest_error) => Err(reqwest_error)
        }
    }

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

        let poll = PollGeneratorApi::create_poll_instance_for_movies(movies);

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

    pub fn create_poll_instance_for_movies(movies: Vec<Movie>) -> PollApiUpsertPayload {
        PollApiUpsertPayload {
            id: None,
            cron: String::from("30 18 * * 1"),
            question: String::from("Quel film ?"),
            answers: movies.iter().map(|m| m.title.clone()).collect(),
            multiselect: true,
            guild:  DISCORD_GUILD,
            channel: DISCORD_CHANNEL,
            duration: 86400
        }
    }
}