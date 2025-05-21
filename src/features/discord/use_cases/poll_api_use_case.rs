use std::env;
use reqwest::{Error};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize};
use uuid::Uuid;
use crate::database::models::{Movie};
use crate::features::discord::poll_domain::PollApiUpsertPayload;

#[derive(Debug, Deserialize, Clone)]
pub struct PollAnswer {
    pub answer: String,
    pub votes: i32
}

pub struct PollApiUseCase {
    base_url: String,
    headers: HeaderMap,
}
impl PollApiUseCase {
    pub fn new() -> Self {
        let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");

        let mut header_map = HeaderMap::new();
        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map.insert(ACCEPT, HeaderValue::from_static("application/json"));

        Self {
            base_url: discord_api_base_url,
            headers: header_map,
        }
    }

    pub async fn initiate_poll_creation(&self, mut poll: PollApiUpsertPayload) -> Result<PollApiUpsertPayload, Error> {
        let url = format!("{base_url}/polls/", base_url = self.base_url);

        let client = reqwest::Client::new();

        let result = match client.post(&url).headers(self.headers.clone()).json(&poll).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let res = match result.json().await {
            Ok(poll) => poll,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        poll.set_id(res);

        Ok(poll)
    }

    pub async fn get_days_from_poll_answers(&self, poll_id: Uuid) -> Result<Vec<String>, Error> {
        let url = format!(
            "{base_url}/polls/{id}/instances/answers",
            base_url = self.base_url,
            id = poll_id
        );

        println!("url: {}", url);

        let client = reqwest::Client::new();
        let response = match client.get(&url).headers(self.headers.clone()).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let answers = match response.json::<Vec<PollAnswer>>().await {
            Ok(answers) => answers,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        println!("{:#?}", answers);

        let mut most_voted_answers: Vec<String> = Vec::new();
        let answer_with_most_vote = answers.iter()
            .enumerate()
            .max_by_key(|(_, p)| p.votes)
            .map(|(_, p)| p);

        if answer_with_most_vote.is_none() {
            return Ok(Vec::new())
        }

        for answer in &answers {
            if answer.votes == answer_with_most_vote.unwrap().votes {
                most_voted_answers.push(answer.answer.clone());
            }
        }

        Ok(most_voted_answers)
    }

    pub async fn _generate_poll_with_movies(movies: Vec<Movie>) -> Result<(), Error> {
        let discord_api_base_url = env::var("DISCORD_API_BASE_URL").expect("Expected DISCORD_API_BASE_URL in the environment");
        let url = String::from(discord_api_base_url) + "/polls";

        let poll = PollApiUseCase::_create_poll_instance_for_movies(movies);

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

    pub fn _create_poll_instance_for_movies(movies: Vec<Movie>) -> PollApiUpsertPayload {
        let discord_guild = env::var("DISCORD_GUILD").expect("Expected DISCORD_GUILD in the environment");
        let discord_channel = env::var("DISCORD_CHANNEL").expect("Expected DISCORD_CHANNEL in the environment");
        PollApiUpsertPayload {
            id: None,
            cron: String::from("30 18 * * 1"),
            question: String::from("Quel film ?"),
            answers: movies.iter().map(|m| m.title.clone()).collect(),
            multiselect: true,
            guild: discord_guild,
            channel: discord_channel,
            duration: 86400,
            onetime: false
        }
    }
}