use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub total_games: i64,
    pub wins: i64,
    pub losses: i64,
    pub rating: f64,
    pub best_rating: f64,
}

impl Player {
    pub async fn create(
        pool: &sqlx::PgPool,
        name: &str,
        rating: Option<f64>,
    ) -> Result<Self, sqlx::Error> {
        let rating = rating.unwrap_or(1000.0);
        sqlx::query_as!(
            Player,
            r#"
            INSERT INTO players (name, rating, best_rating)
            VALUES ($1, $2, $2)
            RETURNING id, name, total_games, wins, losses, rating, best_rating
            "#,
            name,
            rating
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Player,
            r#"
            SELECT id, name, total_games, wins, losses, rating, best_rating
            FROM players
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn leaderboard(
        pool: &sqlx::PgPool,
        limit: Option<i64>,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let limit = limit.unwrap_or(10);
        sqlx::query_as!(
            Player,
            r#"
            SELECT id, name, total_games, wins, losses, rating, best_rating
            FROM players
            ORDER BY rating DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(pool)
        .await
    }
}
