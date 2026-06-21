mod telemetry;

use axum::{
    Json, Router,
    http::{HeaderValue, Method, StatusCode},
    routing::get,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use types::StatusResponse;

#[tokio::main]
async fn main() {
    let _tracer_provider = telemetry::init();

    let web_host = std::env::var("WEB_HOST").expect("WEB_HOST must be set");
    let web_port = std::env::var("WEB_PORT").expect("WEB_PORT must be set");
    let web_origin = format!("http://{web_host}:{web_port}");
    let cors = CorsLayer::new()
        .allow_origin(web_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/alive", get(|| async { StatusCode::OK }))
        .route("/health", get(health))
        .route("/api/status", get(status))
        .layer(cors);

    let host = std::env::var("HOST").expect("HOST must be set");
    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a valid port number");
    let addr = SocketAddr::new(host.parse().expect("HOST must be a valid IP address"), port);
    tracing::info!("Server listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        service: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
