use pyo3::prelude::*;

pub mod styling;
pub mod heatmap;

mod py_element_id;
use py_element_id::{PyElementID, extract_element_id};

mod py_svg_drawing;
use py_svg_drawing::PySvgDrawing;

mod element_type;
use element_type::ElementType;

/// Python module definition
#[pymodule]
fn visualife(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyElementID>()?;
    m.add_class::<PySvgDrawing>()?;
    styling::init_submodule(m)?;
    heatmap::init_submodule(m)?;
    Ok(())
}
