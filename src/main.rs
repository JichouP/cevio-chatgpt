use adapters::{chatgpt::ChatGPTImpl, web::routers::talk_router::talk_router};
use application::services::chatgpt::ChatGPTService;
use axum::{
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::Response,
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;

#[macro_use]
extern crate dotenv_codegen;

mod adapters;
mod application;

#[tokio::main]
async fn main() {
    let chatgpt_adapter = ChatGPTImpl::new();
    let chatgpt_state = Arc::new(ChatGPTService::new(chatgpt_adapter));
    let app: Router = Router::new()
        .with_state(chatgpt_state)
        .nest("/", talk_router())
        .layer(ServiceBuilder::new().layer(from_fn(access_log_on_request)));

    let addr = format!("0.0.0.0:{}", dotenv!("PORT"));

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}
