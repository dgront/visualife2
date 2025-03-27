use pyo3::prelude::*;
use visualife::colors;
/// Python module definition
#[pymodule]
fn visualife_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rgb_to_hex, m)?)?;
    Ok(())
}

#[pyfunction]
fn rgb_to_hex(r: u8, g: u8, b: u8) -> PyResult<String> {
    Ok(visualife::colors::rgb_to_hex(r, g, b))
}