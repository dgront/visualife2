
#[pyclass]
#[derive(Clone)]
pub struct PyStyle { inner: Style, }

#[pymethods]
impl PyStyle {
    #[new]
    fn new() -> Self {
        PyStyle { inner: Style::new(), }
    }

    /// Sets the fill color.
    pub fn with_fill(&self, value: &str) -> Self {
        let mut new = self.inner.clone();
        new.fill = Some(value.to_string());
        PyStyle { inner: new }
    }

    /// Provides the fill color.
    #[getter]
    pub fn get_fill(&self) -> Option<String> { self.inner.fill.clone() }

    /// Sets the fill color.
    #[setter]
    pub fn set_fill(&mut self, value: Option<String>) { self.inner.fill = value; }

    /// Sets the stroke color.
    pub fn with_stroke(&self, value: &str) -> Self {
        let mut new = self.inner.clone();
        new.stroke = Some(value.to_string());
        PyStyle { inner: new }
    }

    /// Provides the stroke color.
    #[getter]
    pub fn get_stroke(&self) -> Option<String> { self.inner.stroke.clone() }

    /// Sets the stroke color.
    #[setter]
    pub fn set_stroke(&mut self, value: Option<String>) { self.inner.stroke = value; }

    /// Sets the stroke width.
    pub fn with_stroke_width(&self, value: f32) -> Self {
        let mut new = self.inner.clone();
        new.stroke_width = Some(value);
        PyStyle { inner: new }
    }

    /// Provides the stroke width.
    #[getter]
    pub fn get_stroke_width(&self) -> Option<f32> { self.inner.stroke_width }

    /// Sets the stroke width.
    #[setter]
    pub fn set_stroke_width(&mut self, value: Option<f32>) { self.inner.stroke_width = value; }

    /// Sets the opacity.
    pub fn with_opacity(&self, value: f32) -> Self {
        let mut new = self.inner.clone();
        new.opacity = Some(value);
        PyStyle { inner: new }
    }

    /// Provides the opacity.
    #[getter]
    pub fn get_opacity(&self) -> Option<f32> { self.inner.opacity }

    /// Sets the opacity.
    #[setter]
    pub fn set_opacity(&mut self, value: Option<f32>) { self.inner.opacity = value; }

    /// Sets the fill opacity.
    pub fn with_fill_opacity(&self, value: f32) -> Self {
        let mut new = self.inner.clone();
        new.fill_opacity = Some(value);
        PyStyle { inner: new }
    }

    /// Provides the fill opacity.
    #[getter]
    pub fn get_fill_opacity(&self) -> Option<f32> { self.inner.fill_opacity }

    /// Sets the fill opacity.
    #[setter]
    pub fn set_fill_opacity(&mut self, value: Option<f32>) { self.inner.fill_opacity = value; }

    /// Sets the stroke opacity.
    pub fn with_stroke_opacity(&self, value: f32) -> Self {
        let mut new = self.inner.clone();
        new.stroke_opacity = Some(value);
        PyStyle { inner: new }
    }

    /// Provides the stroke opacity.
    #[getter]
    pub fn get_stroke_opacity(&self) -> Option<f32> { self.inner.stroke_opacity }

    /// Sets the stroke opacity.
    #[setter]
    pub fn set_stroke_opacity(&mut self, value: Option<f32>) { self.inner.stroke_opacity = value; }

    fn is_empty(&self) -> bool { self.inner.is_empty() }

    fn to_svg(&self) -> String { self.inner.to_svg() }

    fn __repr__(&self) -> PyResult<String> { Ok(self.inner.to_svg()) }
}
