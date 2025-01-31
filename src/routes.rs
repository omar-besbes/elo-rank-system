use crate::handlers::{create_player, report_match, retrieve_player_metrics, root};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/players", post(create_player))
        .route("/matches", post(report_match))
        .route("/leaderboard", get(show_leaderboard))
        .route("/players/:id", get(retrieve_player_metrics))
}
