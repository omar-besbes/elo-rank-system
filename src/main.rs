use axum::{http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tracing_subscriber;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}