use axum::http::Response;
use axum::{
    routing::get,
    Router, response::IntoResponse,
    http::{Request, StatusCode},
};
use axum::extract::{Path, Query, Json};
use axum::debug_handler;
// #[macro_use]axum::debug_handler
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/sleep/:delay", get(sleep))
        .route("/", get(|| async { "Hello, World! -axum" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[debug_handler]
async fn sleep(Path(delay): Path<u64>) -> Result<Json<u64>, StatusCode>  {
    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
    return Ok(Json(delay));
}