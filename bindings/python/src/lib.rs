use pyo3::prelude::*;

mod styling;


/// Python module definition
#[pymodule]
fn visualife(_py: Python, m: &PyModule) -> PyResult<()> {
    styling::init_submodule(_py, m)?;
    Ok(())
}
