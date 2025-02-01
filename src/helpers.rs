use crate::models::Player;

pub fn update_score(
    winner_player: &Player,
    loser_player: &Player,
    k_factor: Option<i64>,
) -> (f64, f64) {
    let k = if let Some(value) = k_factor {
        value as f64
    } else {
        32_f64
    };
    let expected_winner_score =
        1_f64 / (1_f64 + 10_f64.powf((loser_player.rating - winner_player.rating) / 400_f64));
    let expected_loser_score =
        1_f64 / (1_f64 + 10_f64.powf((winner_player.rating - loser_player.rating) / 400_f64));
    let new_winner_rating = winner_player.rating + k * (1_f64 - expected_winner_score);
    let new_loser_rating = loser_player.rating + k * (0_f64 - expected_loser_score);
    (new_winner_rating, new_loser_rating)
}
