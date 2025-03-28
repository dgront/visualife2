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
    pub fn add_style(&mut self, style: &PyStyle) -> u32 {
        self.inner.add_style(style.inner.clone())
    }

    /// Binds a style to an element by ID.
    pub fn style_element(&mut self, style_id: u32, element_id: &PyElementID) {
        self.inner.style_element(style_id, &element_id.inner)
    }

    /// Retrieves the style for the given element.
    ///
    /// Returns `None` if the element has no style.
    pub fn get_style(&self, element_id: &PyElementID) -> Option<PyStyle> {
        self.inner
            .get_style(&element_id.inner)
            .cloned()
            .map(|s| PyStyle { inner: s })
    }

}
