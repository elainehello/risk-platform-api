//! This service never calculates raw indicators
//! It only orchestrates

use axum::Json;
use shared_types::{RiskScoreOutput};
use shared_math::scoring::{compute_risk_score, RiskWeights};
use chrono::Utc;

pub async fn score(
    Json(input): Json<serde_json::Value>
) -> Json<RiskScoreOutput> {

    let weights = RiskWeights {
        regime_weight: 0.4,
        vol_weight: 0.3,
        shock_weight: 0.3,
    };

    let score = compute_risk_score(
        input["regime_score"].as_f64().unwrap(),
        input["vol_score"].as_f64().unwrap(),
        input["shock_score"].as_f64().unwrap(),
        &weights,
    );

    Json(RiskScoreOutput {
        timestamp: Utc::now(),
        risk_score: score,
        margin_multiplier: 1.0 + score / 200.0,
    })
}
