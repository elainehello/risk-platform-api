//! Enterpise-grade deterministic scoring

pub struct RiskWeights {
    pub regime_weight: f64,
    pub vol_weight: f64,
    pub shock_weight: f64,
}

pub fn compute_risk_score(
    regime_score: f64,
    vol_score: f64,
    shock_score: f64,
    weights: &RiskWeights,
) -> f64 {
    let raw_score =
        regime_score * weights.regime_weight +
        vol_score * weights.vol_weight +
        shock_score * weights.shock_weight;

    raw_score.clamp(0.0, 100.0)
}
