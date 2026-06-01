use modernizing_grid_intelligence_without_compromising_nerc_cip_compliance_core::asset_risk_scores;
use numpy::{PyArray1, PyReadonlyArray1, IntoPyArray};
use pyo3::prelude::*;

#[pyfunction]
fn asset_risk_scores_py<'py>(
    py: Python<'py>,
    age: PyReadonlyArray1<f64>,
    criticality: PyReadonlyArray1<f64>,
    exposure: PyReadonlyArray1<f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    Ok(asset_risk_scores(
        age.as_slice()?,
        criticality.as_slice()?,
        exposure.as_slice()?,
    )
    .into_pyarray(py))
}

#[pyfunction]
#[pyo3(signature = (age, criticality, exposure, iterations=10_000))]
fn bench_kernel_py(
    age: PyReadonlyArray1<f64>,
    criticality: PyReadonlyArray1<f64>,
    exposure: PyReadonlyArray1<f64>,
    iterations: usize,
) -> PyResult<f64> {
    let a = age.as_slice()?.to_vec();
    let c = criticality.as_slice()?.to_vec();
    let e = exposure.as_slice()?.to_vec();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = asset_risk_scores(&a, &c, &e);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn modernizing_grid_intelligence_without_compromising_nerc_cip_compliance_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(asset_risk_scores_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}
