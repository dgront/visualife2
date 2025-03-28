use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyType;

use visualife::styling::ColorMap;

#[pyclass(name = "ColorMap")]
pub struct PyColorMap {
    inner: ColorMap,
}

#[pymethods]
impl PyColorMap {
    /// Creates a ColorMap from a list of hex colors and a scalar range.
    #[classmethod]
    pub fn from_range<'py>(_cls: &Bound<'py, PyType>, colors: Vec<String>, from: f64, to: f64) -> PyResult<Self> {
        let refs: Vec<&str> = colors.iter().map(|s| s.as_str()).collect();
        let inner = ColorMap::from_range(&refs, from, to)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Creates a ColorMap from a list of hex colors and corresponding scalar stop positions.
    #[classmethod]
    pub fn from_stops<'py>(_cls: &Bound<'py, PyType>, colors: Vec<String>, stops: Vec<f64>, ) -> PyResult<Self> {
        if colors.len() != stops.len() || colors.len() < 2 {
            return Err(PyValueError::new_err(
                "colors and stops must have the same length and at least 2 points",
            ));
        }

        let refs: Vec<&str> = colors.iter().map(|s| s.as_str()).collect();
        let inner = ColorMap::from_stops(&refs, &stops)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Returns the interpolated hex color for a given scalar value.
    pub fn color(&self, x: f64) -> &str {
        self.inner.color(x)
    }
}
