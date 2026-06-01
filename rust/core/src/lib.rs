//! Composite asset risk score from age, criticality, and exposure vectors.

pub fn asset_risk_scores(age: &[f64], criticality: &[f64], exposure: &[f64]) -> Vec<f64> {
    assert_eq!(age.len(), criticality.len());
    assert_eq!(age.len(), exposure.len());
    age.iter()
        .zip(criticality)
        .zip(exposure)
        .map(|((&a, &c), &e)| (a / 40.0) * 0.4 + c * 0.35 + e * 0.25)
        .collect()
}
