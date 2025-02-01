use crate::{
    helpers::update_score,
    models::{CreatePlayer, LimitParams, Match, Player, ReportMatch},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn root() -> &'static str {
    "Hello, World!"
}

#[axum::debug_handler]
pub async fn create_player(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePlayer>,
) -> (StatusCode, Json<serde_json::Value>) {
    let player_res = Player::create(&pool, &payload.name, payload.rating).await;

    match player_res {
        Ok(player) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "Player added/updated successfully.",
                "player": player
            })),
        ),
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error" : error.to_string()})),
        ),
    }
}

#[axum::debug_handler]
pub async fn report_match(
    State(pool): State<PgPool>,
    Json(payload): Json<ReportMatch>,
) -> (StatusCode, Json<serde_json::Value>) {
    let players = match sqlx::query_as!(
        Player,
        r#"
        SELECT * FROM players 
        WHERE id = $1 OR id = $2
        "#,
        payload.winner_id,
        payload.loser_id,
    )
    .fetch_all(&pool)
    .await
    {
        Ok(players) => players,
        Err(error) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": error.to_string() })),
            )
        }
    };

    if players.len() != 2 {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "One or both players not found" })),
        );
    }

    let (winner_player, loser_player) = {
        let winner = players.iter().find(|player| player.id == payload.winner_id);
        let loser = players.iter().find(|player| player.id == payload.loser_id);

        match (winner, loser) {
            (Some(w), Some(l)) => (w, l),
            _ => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Could not identify both players" })),
                )
            }
        }
    };

    let match_res =
        Match::report(&pool, payload.winner_id, payload.loser_id, payload.k_factor).await;

    if let Err(error) = match_res {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": error.to_string()})),
        );
    }

    let (winner_new_rating, loser_new_rating) =
        update_score(winner_player, loser_player, payload.k_factor);

    let res = sqlx::query!(
        r#"
            UPDATE players
            SET 
                rating = CASE 
                    WHEN id = $1 THEN $3::DOUBLE PRECISION
                    WHEN id = $2 THEN $4::DOUBLE PRECISION
                END,
                best_rating = GREATEST(
                    best_rating, 
                    CASE 
                        WHEN id = $1 THEN $3::DOUBLE PRECISION
                        WHEN id = $2 THEN $4::DOUBLE PRECISION
                    END
                )
            WHERE id IN ($1, $2)
            "#,
        payload.winner_id,
        payload.loser_id,
        winner_new_rating,
        loser_new_rating,
    )
    .execute(&pool)
    .await;

    match res {
        Ok(result) => {
            if result.rows_affected() == 2 {
                (
                    StatusCode::CREATED,
                    Json(json!({
                        "message": "Match processed successfully.",
                        "winnerRating": winner_new_rating,
                        "loserRating": loser_new_rating,
                    })),
                )
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "One or both players not found" })),
                )
            }
        }
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": error.to_string() })),
        ),
    }
}

#[axum::debug_handler]
pub async fn show_leaderboard(
    Query(params): Query<LimitParams>,
    State(pool): State<PgPool>,
) -> (StatusCode, Json<serde_json::Value>) {
    let limit = params.limit;
    let res = Player::leaderboard(&pool, limit).await;

    match res {
        Err(error) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": error.to_string() })),
        ),
        Ok(players) => (StatusCode::OK, Json(json!(players))),
    }
}

#[axum::debug_handler]
pub async fn retrieve_player_metrics(
    Path(user_id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> (StatusCode, Json<serde_json::Value>) {
    let player_res = Player::find_by_id(&pool, user_id).await;

    match player_res {
        Ok(player) => (StatusCode::OK, Json(json!(player))),
        Err(error) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": error.to_string() })),
        ),
    }
}
