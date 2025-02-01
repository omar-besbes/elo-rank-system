use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub rating: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct LimitParams {
    pub limit: Option<i64>,
}
