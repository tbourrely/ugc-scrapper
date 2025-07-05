use std::env;
use reqwest::{Error};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize};
use uuid::Uuid;
use crate::database::models::{Poll, PollType};
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

    pub async fn initiate_poll_creation(&self, poll: PollApiUpsertPayload, poll_type: PollType) -> Result<Poll, Error> {
        let url = format!("{base_url}/poll-groups", base_url = self.base_url);

        let client = reqwest::Client::new();

        let result = match client.post(&url).headers(self.headers.clone()).json(&poll).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let res = match result.json().await {
            Ok(poll) => poll,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let domain_poll: Poll = poll.transform(res, poll_type);

        Ok(domain_poll)
    }

    pub async fn get_most_voted_answers_from_poll_answers(&self, poll_id: Uuid) -> Result<Vec<String>, Error> {
        let url = format!(
            "{base_url}/poll-groups/{id}/instances/answers",
            base_url = self.base_url,
            id = poll_id
        );

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
}