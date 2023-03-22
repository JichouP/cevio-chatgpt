use crate::application::services::chatgpt::ChatGPTAdapter;
use axum::async_trait;
use chatgpt::{
    err::Error,
    prelude::{ChatGPT, Conversation},
};

#[derive(Clone)]
pub struct ChatGPTImpl {
    client: ChatGPT,
    conversation: Conversation,
    updated_at: std::time::Instant,
}

impl ChatGPTImpl {
    pub fn new() -> Self {
        let client = ChatGPT::new(dotenv!("CHATGPT_API_KEY")).unwrap();
        let conversation = client.new_conversation();
        Self {
            client,
            conversation,
            updated_at: std::time::Instant::now(),
        }
    }
}

#[async_trait]
impl ChatGPTAdapter for ChatGPTImpl {
    async fn chat(&mut self, text: &str) -> Result<String, Error> {
        // If more than 600 seconds have passed, create a new chat
        if std::time::Instant::now()
            .duration_since(self.updated_at)
            .as_secs()
            > 600
        {
            self.conversation = self.client.new_conversation();
        }
        self.updated_at = std::time::Instant::now();
        let res = self.conversation.send_message(text).await?;
        Ok(res.message().content.clone())
    }
}
