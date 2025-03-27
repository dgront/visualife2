use pyo3::prelude::*;
use pyo3::types::{PyString, PyLong};
use pyo3::exceptions::PyTypeError;
use visualife::basic_shapes::ElementID;

#[pyclass(name = "ElementID")]
#[derive(Clone)]
pub struct PyElementID {
    pub inner: ElementID,
}

#[pymethods]
impl PyElementID {
    /// Creates an ElementID from a string or an integer.
    #[new]
    fn new(value: &PyAny) -> PyResult<Self> {
        if let Ok(s) = value.extract::<&str>() {
            Ok(PyElementID { inner: ElementID::from(s) })
        } else if let Ok(i) = value.extract::<i32>() {
            Ok(PyElementID { inner: ElementID::from(i) })
        } else {
            Err(PyTypeError::new_err("ElementID must be created from str or int"))
        }
    }

    /// Returns a new ElementID with a prefix added to the current ID.
    pub fn new_with_prefix(&self, prefix: &str) -> Self {
        PyElementID {
            inner: self.inner.new_with_prefix(prefix),
        }
    }

    /// Returns the string representation of the ID.
    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    /// Debug-style string representation.
    fn __repr__(&self) -> String {
        format!("ElementID('{}')", self.inner.to_string())
    }

    /// Optional: compare equality
    fn __richcmp__(&self, other: PyRef<PyElementID>, op: pyo3::basic::CompareOp) -> Py<PyAny> {
        Python::with_gil(|py| {
            match op {
                pyo3::basic::CompareOp::Eq => (self.inner.id == other.inner.id).into_py(py),
                pyo3::basic::CompareOp::Ne => (self.inner.id != other.inner.id).into_py(py),
                _ => py.NotImplemented(),
            }
        })
    }
}
