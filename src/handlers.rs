use crate::models::{CreatePlayer, LimitParams, Match, Player, ReportMatch};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde_json::json;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_player(
    Json(payload): Json<CreatePlayer>,
) -> (StatusCode, Json<serde_json::Value>) {
    let player = Player {
        id: "1337".to_string(),
        name: payload.name,
        total_games: 0,
        losses: 0,
        wins: 0,
        rating: payload.rating.unwrap_or(1000),
        best_rating: payload.rating.unwrap_or(1000),
    };

    (
        StatusCode::CREATED,
        Json(json!({
            "message": "Player added/updated successfully.",
            "player": player
        })),
    )
}

pub async fn report_match(
    Json(payload): Json<ReportMatch>,
) -> (StatusCode, Json<serde_json::Value>) {
    let _match = Match {
        id: "1337".to_string(),
        winner_id: payload.winner_id,
        loser_id: payload.loser_id,
        k_factor: payload.k_factor.unwrap_or(32),
    };

    (
        StatusCode::CREATED,
        Json(json!({
            "message": "Match processed successfully.",
            "winnerRating": 0,
            "loserRating": 0,
        })),
    )
}

pub async fn show_leaderboard(
    Query(params): Query<LimitParams>,
) -> (StatusCode, Json<serde_json::Value>) {
    let limit = params.limit;
    let players = [
        Player {
            id: "1337".to_string(),
            name: payload.name,
            total_games: 0,
            losses: 0,
            wins: 0,
            rating: 1000,
            best_rating: 1000,
        },
        Player {
            id: "1337".to_string(),
            name: payload.name,
            total_games: 0,
            losses: 0,
            wins: 0,
            rating: 1000,
            best_rating: 1000,
        },
    ];

    (StatusCode::OK, Json(json!(players)))
}

pub async fn retrieve_player_metrics(
    Path(user_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    let player = Player {
        id: user_id,
        name: payload.name,
        total_games: 0,
        losses: 0,
        wins: 0,
        rating: 1000,
        best_rating: 1000,
    };

    (StatusCode::OK, Json(json!(player)))
}
