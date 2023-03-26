use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    adapters::{chatgpt::ChatGPTImpl, web::controllers::talk_controller::post_talk},
    application::services::chatgpt::ChatGPTService,
};

pub fn talk_router() -> Router<Arc<RwLock<ChatGPTService<ChatGPTImpl>>>> {
    Router::new().nest("/talk", talk_routes())
}

fn talk_routes() -> Router<Arc<RwLock<ChatGPTService<ChatGPTImpl>>>> {
    Router::new().route("/", get(post_talk))
}
