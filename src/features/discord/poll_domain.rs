use crate::database::models::{Poll, PollType};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

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
        let discord_guild =
            env::var("DISCORD_GUILD").expect("Expected DISCORD_GUILD in the environment");
        let discord_channel =
            env::var("DISCORD_CHANNEL").expect("Expected DISCORD_CHANNEL in the environment");
        PollApiUpsertPayload {
            cron,
            question,
            answers,
            multiselect: true,
            guild: discord_guild,
            channel: discord_channel,
            duration: 86400,
            onetime: false,
        }
    }

    pub fn transform(self, distant_id: Uuid, poll_type: PollType) -> Poll {
        Poll::new(distant_id, poll_type, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_new_method_sets_fields_correctly() {
        // Set environment variables for the test
        unsafe {
            env::set_var("DISCORD_GUILD", "test_guild_id");
            env::set_var("DISCORD_CHANNEL", "test_channel_id");
        }

        let cron = "0 0 * * *".to_string();
        let question = "What is your favorite color?".to_string();
        let answers = vec!["Red".to_string(), "Blue".to_string(), "Green".to_string()];

        let payload = PollApiUpsertPayload::new(cron.clone(), question.clone(), answers.clone());

        assert_eq!(payload.cron, cron);
        assert_eq!(payload.question, question);
        assert_eq!(payload.answers, answers);
        assert_eq!(payload.multiselect, true);
        assert_eq!(payload.guild, "test_guild_id");
        assert_eq!(payload.channel, "test_channel_id");
        assert_eq!(payload.duration, 86400);
        assert_eq!(payload.onetime, false);
    }
}
