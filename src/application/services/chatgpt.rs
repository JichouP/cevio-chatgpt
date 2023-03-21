use axum::async_trait;
use reqwest::Error;

#[async_trait]
pub trait ChatGPTService {
    async fn new_chat(&self, text: &str) -> Result<String, Error>;
    async fn conversation(&self, text: &str, ctx: &str) -> Result<String, Error>;
}
