use anyhow::Result;
use axum::async_trait;

#[async_trait]
pub trait ChatGPTAdapter {
    async fn chat(&mut self, text: &str) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct ChatGPTService<T: ChatGPTAdapter> {
    chatgpt_adapter: T,
}

impl<T: ChatGPTAdapter> ChatGPTService<T> {
    pub fn new(chatgpt_adapter: T) -> Self {
        Self { chatgpt_adapter }
    }

    pub async fn chat(&mut self, text: &str) -> Result<String> {
        self.chatgpt_adapter.chat(text).await
    }
}
