use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use std::collections::HashMap;

use crate::{adapters::chatgpt::ChatGPTImpl, application::services::chatgpt::ChatGPTService};

#[axum::debug_handler]
pub async fn post_talk(
    Query(params): Query<HashMap<String, String>>,
    State(chatgpt_state): State<ChatGPTService<ChatGPTImpl>>,
) -> Result<StatusCode, StatusCode> {
    let text = params.get("text").ok_or(StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::OK)
}
