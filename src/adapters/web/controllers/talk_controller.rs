use crate::{adapters::chatgpt::ChatGPTImpl, application::services::chatgpt::ChatGPTService};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[axum::debug_handler]
pub async fn post_talk(
    Query(params): Query<HashMap<String, String>>,
    State(chatgpt_state): State<Arc<RwLock<ChatGPTService<ChatGPTImpl>>>>,
) -> Result<String, StatusCode> {
    let text = params.get("text").ok_or(StatusCode::BAD_REQUEST)?;
    let mut chatgpt_state = chatgpt_state.write().await;
    let res = chatgpt_state
        .chat(text)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("質問: {text:?}",);
    println!("返答: {res:?}",);

    reqwest::Client::new()
        .get("http://localhost:50080/talk")
        .query(&[("text", &res)])
        .send()
        .await
        .unwrap();

    Ok(res)
}
