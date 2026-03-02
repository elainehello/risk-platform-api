//! All services communicate using strict types

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub risk_profile_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegimeOutput {
    pub regime: RegimeState,
    pub confidence: f64,
    pub model_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RegimeState {
    Normal,
    RiskOffInitial,
    StructuralStress,
    LiquidityEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolatilityOutput {
    pub projected_24h_vol: f64,
    pub dispersion_index: f64,
    pub model_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShockOutput {
    pub shock_severity: f64,
    pub half_life_days: f64,
    pub model_version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RiskScoreOutput {
    pub timestamp: DateTime<Utc>,
    pub risk_score: f64,
    pub margin_multiplier: f64,
}
