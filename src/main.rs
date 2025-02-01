use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tracing_subscriber;

mod handlers;
mod models;
mod routes;
mod helpers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
