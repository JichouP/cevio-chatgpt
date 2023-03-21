use axum::{extract::Query, http::StatusCode};
use std::collections::HashMap;

pub async fn post_talk(
    Query(params): Query<HashMap<String, String>>,
) -> Result<StatusCode, StatusCode> {
    let text = params.get("text").ok_or(StatusCode::BAD_REQUEST)?;

    Ok(StatusCode::OK)
}
