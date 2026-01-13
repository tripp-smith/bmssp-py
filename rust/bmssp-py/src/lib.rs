use pyo3::prelude::*;

mod sssp;

#[pymodule]
fn _bmssp(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sssp::sssp_f32_csr, m)?)?;
    m.add_function(wrap_pyfunction!(sssp::sssp_f64_csr, m)?)?;
    Ok(())
}
