use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::services::chatgpt::ChatGPTAdapter;
use anyhow::Result;
use axum::async_trait;
use chatgpt::prelude::{ChatGPT, Conversation};

#[derive(Clone)]
pub struct ChatGPTImpl {
    client: ChatGPT,
    conversation: Arc<RwLock<Conversation>>,
    updated_at: std::time::Instant,
}

impl ChatGPTImpl {
    pub fn new() -> Self {
        let client = ChatGPT::new(dotenv!("CHATGPT_API_KEY")).unwrap();
        let conversation = client.new_conversation();
        Self {
            client,
            conversation: Arc::new(RwLock::new(conversation)),
            updated_at: std::time::Instant::now(),
        }
    }
}

#[async_trait]
impl ChatGPTAdapter for ChatGPTImpl {
    async fn chat(&mut self, text: &str) -> Result<String> {
        // If more than 600 seconds have passed, create a new chat
        if std::time::Instant::now()
            .duration_since(self.updated_at)
            .as_secs()
            > 600
        {
            self.conversation = Arc::new(RwLock::new(self.client.new_conversation()));
        }
        self.updated_at = std::time::Instant::now();
        let res = self.conversation.write().await.send_message(text).await?;
        Ok(res.message().content.clone())
    }
}
