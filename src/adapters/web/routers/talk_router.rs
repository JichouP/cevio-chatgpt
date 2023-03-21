use axum::{routing::post, Router};

use crate::adapters::web::controllers::talk_controller::post_talk;

pub fn talk_router() -> Router {
    Router::new().nest("/talk", talk_routes())
}

fn talk_routes() -> Router {
    Router::new().route("/", post(post_talk))
}
