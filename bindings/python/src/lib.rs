use pyo3::prelude::*;

/// Python module definition
#[pymodule]
fn visualife_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rgb_to_hex, m)?)?;
    Ok(())
}

#[pyfunction]
fn rgb_to_hex(r: u8, g: u8, b: u8) -> PyResult<String> {
    Ok(visualife::styling::rgb_to_hex(r, g, b))
}