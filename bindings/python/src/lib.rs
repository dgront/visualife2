use pyo3::prelude::*;

pub mod styling;

mod py_element_id;
use py_element_id::{PyElementID, extract_element_id};

mod py_svg_drawing;
use py_svg_drawing::PySvgDrawing;

mod element_type;
use element_type::ElementType;

/// Python module definition
#[pymodule]
fn visualife(_py: Python, m: &PyModule) -> PyResult<()> {
    styling::init_submodule(_py, m)?;
    m.add_class::<PyElementID>()?;
    m.add_class::<PySvgDrawing>()?;
    Ok(())
}
