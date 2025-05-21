use std::env;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::models::{Poll, PollType};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollApiUpsertPayload {
    pub id: Option<Uuid>,
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
    pub fn new(id: Option<Uuid>, cron: String, question: String, answers: Vec<String>) -> Self {
        let discord_guild = env::var("DISCORD_GUILD").expect("Expected DISCORD_GUILD in the environment");
        let discord_channel = env::var("DISCORD_CHANNEL").expect("Expected DISCORD_CHANNEL in the environment");
        PollApiUpsertPayload {
            id,
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

    pub fn set_id(&mut self, id: Uuid) {
        self.id = Some(id);
    }

    pub fn transform(self) -> Poll {
        Poll::new(self.id.unwrap(), PollType::SelectDay, None)
    }
}