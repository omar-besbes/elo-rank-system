use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReportMatch {
    pub winner_id: String,
    pub loser_id: String,
    pub k_factor: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub id: String,
    pub winner_id: String,
    pub loser_id: String,
    pub k_factor: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub rating: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub total_games: u64,
    pub wins: u64,
    pub losses: u64,
    pub rating: i64,
    pub best_rating: i64,
}

#[derive(Serialize, Deserialize)]
pub struct LimitParams {
    pub limit: Option<u64>,
}
