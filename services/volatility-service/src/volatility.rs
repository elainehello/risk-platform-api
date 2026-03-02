//! Handles vol projections only

pub fn projected_vol(vix: f64, credit_spread: f64) -> f54 {
    (vix * 0.6 + credit_spread * 0.4) * 1.05
}