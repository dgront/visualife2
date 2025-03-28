use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::FromPyObject;

#[derive(Clone, Copy, Debug)]
pub enum ElementType {
    Line,
    Rect,
    Circle,
    Ellipse,
    Polygon,
    Polyline,
    Path,
    Text,
    Group,
}

impl<'py> FromPyObject<'py> for ElementType {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let s = ob.extract::<&str>()?;
        match s.to_lowercase().as_str() {
            "line" => Ok(ElementType::Line),
            "rect" => Ok(ElementType::Rect),
            "circle" => Ok(ElementType::Circle),
            "ellipse" => Ok(ElementType::Ellipse),
            "polygon" => Ok(ElementType::Polygon),
            "polyline" => Ok(ElementType::Polyline),
            "path" => Ok(ElementType::Path),
            "text" => Ok(ElementType::Text),
            "group" => Ok(ElementType::Group),
            _ => Err(PyValueError::new_err("Invalid ElementType")),
        }
    }
}