use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::models::{Poll, PollType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollApiUpsertPayload {
    pub cron: String,
    pub question: String,
    pub answers: Vec<String>,
    pub multiselect: bool,
    pub guild: String,
    pub channel: String,
    pub duration: i32,
    pub onetime: bool,
}

impl PollApiUpsertPayload {
    pub fn new(cron: String, question: String, answers: Vec<String>) -> Self {
        let discord_guild = env::var("DISCORD_GUILD").expect("Expected DISCORD_GUILD in the environment");
        let discord_channel = env::var("DISCORD_CHANNEL").expect("Expected DISCORD_CHANNEL in the environment");
        PollApiUpsertPayload {
            cron,
            question,
            answers,
            multiselect: true,
            guild: discord_guild,
            channel: discord_channel,
            duration: 86400,
            onetime: false
        }
    }

    pub fn transform(self, distant_id: Uuid, poll_type: PollType) -> Poll {
        Poll::new(distant_id, poll_type, None)
    }
}