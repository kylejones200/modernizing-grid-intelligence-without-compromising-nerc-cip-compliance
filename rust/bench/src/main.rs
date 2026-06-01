use modernizing_grid_intelligence_without_compromising_nerc_cip_compliance_core::asset_risk_scores;

fn main() {
    let n = 5000usize;
    let age: Vec<f64> = (0..n).map(|i| (i % 40) as f64 + 1.0).collect();
    let crit: Vec<f64> = (0..n).map(|i| (i % 10) as f64 * 0.1).collect();
    let exp: Vec<f64> = (0..n).map(|i| (i % 7) as f64 * 0.12).collect();
    for _ in 0..10000 {
        let _ = asset_risk_scores(&age, &crit, &exp);
    }
}
