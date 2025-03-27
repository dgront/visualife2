use pyo3::prelude::*;

mod styling;

mod py_element_id;
use py_element_id::PyElementID;

/// Python module definition
#[pymodule]
fn visualife(_py: Python, m: &PyModule) -> PyResult<()> {
    styling::init_submodule(_py, m)?;
    m.add_class::<PyElementID>()?;
    Ok(())
}
