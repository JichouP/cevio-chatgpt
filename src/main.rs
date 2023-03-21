use adapters::web::routers::talk_router::talk_router;
use axum::{
    http::{Request, StatusCode},
    middleware::{from_fn, Next},
    response::Response,
    Router,
};
use tower::ServiceBuilder;

mod adapters;
mod application;

const PORT: i32 = 3000;

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .nest("/", talk_router())
        .layer(ServiceBuilder::new().layer(from_fn(access_log_on_request)));

    let addr = format!("0.0.0.0:{PORT}");

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn access_log_on_request<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    println!("{} {}", req.method(), req.uri());
    Ok(next.run(req).await)
}
