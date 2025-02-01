use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Serialize, Deserialize, FromRow)]
pub struct ReportMatch {
    pub winner_id: Uuid,
    pub loser_id: Uuid,
    pub k_factor: Option<i64>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Match {
    pub id: Uuid,
    pub winner_id: Uuid,
    pub loser_id: Uuid,
    pub k_factor: i64,
}

impl Match {
    pub async fn report(
        pool: &sqlx::PgPool,
        winner_id: Uuid,
        loser_id: Uuid,
        k_factor: Option<i64>,
    ) -> Result<(), sqlx::Error> {
        let k_factor = k_factor.unwrap_or(32);
        // TODO: Implement Elo rating calculation and update player ratings
        // This is a placeholder for the actual logic
        sqlx::query!(
            r#"
            INSERT INTO matches (winner_id, loser_id, k_factor)
            VALUES ($1, $2, $3)
            "#,
            winner_id,
            loser_id,
            k_factor
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
