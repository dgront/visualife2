use pyo3::prelude::*;

use visualife::styling::{StyleManager};

use crate::PyElementID;
use crate::styling::{PyStyle};

#[pyclass(name = "StyleManager")]
pub struct PyStyleManager { inner: StyleManager, }

#[pymethods]
impl PyStyleManager {
    /// Creates a new StyleManager.
    #[new]
    fn new() -> Self {
        PyStyleManager { inner: StyleManager::new(), }
    }

    /// Adds a style to the manager. Returns a style ID.
    pub fn define_style(&mut self, style: &PyStyle) -> usize {
        self.inner.define_style(style.inner.clone())
    }

    /// Binds a style to an element by ID.
    pub fn style_element(&mut self, style_id: usize, element_id: PyElementID) {
        self.inner.style_element(style_id, element_id.inner)
    }

    pub fn get_style_id(&self, element_id: &PyElementID) -> Option<usize> {
        self.inner.get_style_id(&element_id.inner)
    }

    /// Retrieves the style for the given element.
    ///
    /// Returns `None` if the element has no style.
    pub fn get_style(&self, style_id: usize) -> PyStyle {
        let style = self.inner.get_style(style_id);
        PyStyle { inner: style.clone() }
    }

}
