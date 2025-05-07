use visualife::styling::Style;
use pyo3::prelude::*;

use std::collections::HashSet;
use pyo3::types::{PyDict, PyDictMethods};


#[pyclass(name = "Style")]
#[derive(Clone)]
pub struct PyStyle { pub(crate) inner: Style, }

#[pymethods]
impl PyStyle {
    #[new]
    fn new() -> Self {
        PyStyle { inner: Style::new(), }
    }

    /// Provides the fill color.
    #[getter]
    pub fn get_fill(&self) -> Option<String> { self.inner.fill.clone() }

    /// Sets the fill color.
    #[setter]
    pub fn set_fill(&mut self, value: Option<String>) { self.inner.fill = value; }

    /// Provides the stroke color.
    #[getter]
    pub fn get_stroke(&self) -> Option<String> { self.inner.stroke.clone() }

    /// Sets the stroke color.
    #[setter]
    pub fn set_stroke(&mut self, value: Option<String>) { self.inner.stroke = value; }

    /// Provides the stroke width.
    #[getter]
    pub fn get_stroke_width(&self) -> Option<f32> { self.inner.stroke_width }

    /// Sets the stroke width.
    #[setter]
    pub fn set_stroke_width(&mut self, value: Option<f32>) { self.inner.stroke_width = value; }

    /// Provides the opacity.
    #[getter]
    pub fn get_opacity(&self) -> Option<f32> { self.inner.opacity }

    /// Sets the opacity.
    #[setter]
    pub fn set_opacity(&mut self, value: Option<f32>) { self.inner.opacity = value; }

    /// Provides the fill opacity.
    #[getter]
    pub fn get_fill_opacity(&self) -> Option<f32> { self.inner.fill_opacity }

    /// Sets the fill opacity.
    #[setter]
    pub fn set_fill_opacity(&mut self, value: Option<f32>) { self.inner.fill_opacity = value; }

    /// Provides the stroke opacity.
    #[getter]
    pub fn get_stroke_opacity(&self) -> Option<f32> { self.inner.stroke_opacity }

    /// Sets the stroke opacity.
    #[setter]
    pub fn set_stroke_opacity(&mut self, value: Option<f32>) { self.inner.stroke_opacity = value; }

    /// Gets the text anchor.
    #[getter]
    pub fn get_text_anchor(&self) -> Option<String> {
        self.inner.text_anchor.clone()
    }

    /// Sets the text anchor.
    #[setter]
    pub fn set_text_anchor(&mut self, value: Option<String>) {
        self.inner.text_anchor = value;
    }

    /// Gets the font family.
    #[getter]
    pub fn get_font_family(&self) -> Option<String> {
        self.inner.font_family.clone()
    }

    /// Sets the font family.
    #[setter]
    pub fn set_font_family(&mut self, value: Option<String>) {
        self.inner.font_family = value;
    }

    /// Gets the font size.
    #[getter]
    pub fn get_font_size(&self) -> Option<String> {
        self.inner.font_size.clone()
    }

    /// Sets the font size.
    #[setter]
    pub fn set_font_size(&mut self, value: Option<String>) {
        self.inner.font_size = value;
    }

    /// Gets the font weight.
    #[getter]
    pub fn get_font_weight(&self) -> Option<String> {
        self.inner.font_weight.clone()
    }

    /// Sets the font weight.
    #[setter]
    pub fn set_font_weight(&mut self, value: Option<String>) {
        self.inner.font_weight = value;
    }

    /// Gets the text decoration.
    #[getter]
    pub fn get_text_decoration(&self) -> Option<String> {
        self.inner.text_decoration.clone()
    }

    /// Sets the text decoration.
    #[setter]
    pub fn set_text_decoration(&mut self, value: Option<String>) {
        self.inner.text_decoration = value;
    }

    /// Gets the dominant baseline.
    #[getter]
    pub fn get_dominant_baseline(&self) -> Option<String> {
        self.inner.dominant_baseline.clone()
    }

    /// Sets the dominant baseline.
    #[setter]
    pub fn set_dominant_baseline(&mut self, value: Option<String>) {
        self.inner.dominant_baseline = value;
    }

    fn is_empty(&self) -> bool { self.inner.is_empty() }

    fn to_svg(&self) -> String { self.inner.to_svg() }

    fn __repr__(&self) -> PyResult<String> { Ok(self.inner.to_svg()) }
}

#[pyfunction]
#[pyo3(signature = (base_style=None, **kwargs))]
pub fn style<'py>(
    py: Python<'py>,
    base_style: Option<PyStyle>,
    kwargs: Option<&Bound<'py, PyDict>>,
) -> PyResult<PyStyle> {

    let mut style = base_style.unwrap_or(PyStyle { inner: Style::new() });

    let valid_keys: HashSet<&str> = [
        "fill", "stroke", "stroke_width", "opacity",
        "fill_opacity", "stroke_opacity", "text_anchor",
        "font_family", "font_size", "font_weight",
        "text_decoration", "dominant_baseline"
    ].iter().cloned().collect();

    if let Some(dict) = kwargs {
        for (k, v) in dict.iter() {
            let key = k.extract::<&str>()?;

            if !valid_keys.contains(key) {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid style key: '{}'", key),
                ));
            }

            match key {
                "fill" => style.inner.fill = Some(v.extract::<&str>()?.to_string()),
                "stroke" => style.inner.stroke = Some(v.extract::<&str>()?.to_string()),
                "stroke_width" => style.inner.stroke_width = Some(v.extract::<f32>()?),
                "opacity" => style.inner.opacity = Some(v.extract::<f32>()?),
                "fill_opacity" => style.inner.fill_opacity = Some(v.extract::<f32>()?),
                "stroke_opacity" => style.inner.stroke_opacity = Some(v.extract::<f32>()?),
                "text_anchor" => style.inner.text_anchor = Some(v.extract::<&str>()?.to_string()),
                "font_family" => style.inner.font_family = Some(v.extract::<&str>()?.to_string()),
                "font_size" => style.inner.font_size = Some(v.extract::<&str>()?.to_string()),
                "font_weight" => style.inner.font_weight = Some(v.extract::<&str>()?.to_string()),
                "text_decoration" => style.inner.text_decoration = Some(v.extract::<&str>()?.to_string()),
                "dominant_baseline" => style.inner.dominant_baseline = Some(v.extract::<&str>()?.to_string()),
                _ => {
                    return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("Invalid style key: '{}'", key),
                    ));
                }
            }
        }
    }

    Ok(style)
}
