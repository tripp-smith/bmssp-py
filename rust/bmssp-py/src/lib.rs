use pyo3::prelude::*;

mod sssp;

#[pymodule]
fn _bmssp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sssp::sssp_f32_csr, m)?)?;
    m.add_function(wrap_pyfunction!(sssp::sssp_f64_csr, m)?)?;
    Ok(())
}
