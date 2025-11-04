use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyIOError};
use pyo3::types::PyTuple;
use pyo3::types::{PyDict, PyDictMethods};


use crate::{extract_element_id};
use crate::ElementType;
use crate::styling::{PyStyle};

use visualife::{SvgDrawing, ElementID};
use visualife::basic_shapes::{SvgElement};

#[pyclass(name = "SvgDrawing")]
pub struct PySvgDrawing {
    inner: SvgDrawing,
}

#[pymethods]
impl PySvgDrawing {
    #[new]
    fn new(width: f32, height: f32) -> Self {
        PySvgDrawing { inner: SvgDrawing::new(width, height), }
    }

    #[getter]
    fn get_width(&self) -> f32 { self.inner.width() }

    #[getter]
    fn get_height(&self) -> f32 { self.inner.height() }

    /// Draws the SVG to stdout.
    fn to_svg(&mut self) -> String { self.inner.to_svg() }

    fn save_svg(&self, fname: &str) -> PyResult<()> {
        self.inner.save_svg(fname).map_err(|e| PyIOError::new_err(e.to_string()))
    }

    /// Adds an element by type and arguments.
    #[pyo3(signature = (element_type, id, args, **kwargs))]
    fn add_element<'py>(&mut self, element_type: ElementType, id: &Bound<'py, PyAny>,
                        args: &Bound<'py, PyTuple>, kwargs: Option<&Bound<'py, PyDict>>) -> PyResult<()> {

        let id = extract_element_id(id)?;
        let el = Self::create_element(element_type, id, args, kwargs)?;

        self.inner.add_element(el);

        Ok(())
    }

    /// Adds a new SVG element to a group identified by group_id.
    #[pyo3(signature = (group_id, element_type, id, args, **kwargs))]
    fn add_element_to_group<'py>( &mut self, group_id: &Bound<'py, PyAny>,
                element_type: ElementType, id: &Bound<'py, PyAny>,
                args: &Bound<'py, PyTuple>, kwargs: Option<&Bound<'py, PyDict>> ) -> PyResult<()> {

        let id = extract_element_id(id)?;
        let group_id = extract_element_id(group_id)?;
        let el = Self::create_element(element_type, id, args, kwargs)?;

        self.inner.add_element_to_group(el, &group_id).map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(())
    }
}


impl PySvgDrawing {

    fn create_element<'py>(element_type: ElementType, id: ElementID,
                        args: &Bound<'py, PyTuple>, kwargs: Option<&Bound<'py, PyDict>>) -> PyResult<SvgElement> {

        let mut el = match element_type {
            ElementType::Line => Self::parse_line(&id, args),
            ElementType::Rect => Self::parse_rect(&id, args),
            ElementType::Circle => Self::parse_circle(&id, args),
            ElementType::Ellipse => Self::parse_ellipse(&id, args),
            ElementType::Polygon => Self::parse_polygon(&id, args),
            ElementType::Polyline => Self::parse_polyline(&id, args),
            ElementType::Path => Self::parse_path(&id, args),
            ElementType::Text => Self::parse_text(&id, args),
            ElementType::Group => Self::parse_group(&id),
        }?;

        if let Some(kwargs) = kwargs {
            if let Ok(Some(style_obj)) = kwargs.get_item("style") {
                let style_py = style_obj.extract::<PyStyle>()?;
                el.set_style(style_py.inner);
            }
        }

        return Ok(el);
    }

    fn parse_line<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let [x1, y1, x2, y2] = extract_f32_args::<4>(args)?;
        Ok(SvgElement::line(id.clone(), x1, y1, x2, y2 ))
    }

    fn parse_circle<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let [cx, cy, r] = extract_f32_args::<3>(args)?;
        Ok(SvgElement::circle(id.clone(), cx, cy, r ))
    }

    fn parse_rect<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let [x, y, width, height] = extract_f32_args::<4>(args)?;
        Ok(SvgElement::rect(id.clone(), x, y, width, height ))
    }

    fn parse_ellipse<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let [cx, cy, rx, ry] = extract_f32_args::<4>(args)?;
        Ok(SvgElement::ellipse(id.clone(), cx, cy, rx, ry ))
    }

    fn parse_polygon<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let points = extract_point_list(args)?;
        Ok(SvgElement::polygon(id.clone(), points))
    }

    fn parse_polyline<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        let points = extract_point_list(args)?;
        Ok(SvgElement::polyline(id.clone(), points))
    }

    fn parse_group(id: &ElementID) -> PyResult<SvgElement> {

        Ok(SvgElement::group(id.clone(), Vec::new()))
    }

    fn parse_text<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        if args.len() != 3 {
            return Err(PyValueError::new_err("Text requires 3 arguments: x, y, content"));
        }

        let x = args.get_item(0)?.extract::<f32>()?;
        let y = args.get_item(1)?.extract::<f32>()?;
        let content = args.get_item(2)?.extract::<String>()?;

        Ok(SvgElement::text(id.clone(), x, y, content))
    }

    fn parse_path<'py>(id: &ElementID, args: &Bound<'py, PyTuple>) -> PyResult<SvgElement> {
        if args.len() != 1 {
            return Err(PyValueError::new_err("Path requires 1 argument: d (string)"));
        }
        let d = args.get_item(0)?.extract::<String>()?;
        Ok(SvgElement::path(id.clone(), d ))
    }
}

fn extract_f32_args<const N: usize>(args: &Bound<'_, PyTuple>) -> PyResult<[f32; N]> {
    if args.len() != N {
        return Err(PyValueError::new_err(format!("Expected {N} arguments")));
    }
    let mut result = [0.0; N];
    for i in 0..N {
        result[i] = args.get_item(i)?.extract::<f32>()?;
    }
    Ok(result)
}

/// Extracts a list of (f32, f32) pairs from a PyTuple.
/// Each item must be a 2-element tuple of floats.
fn extract_point_list<'py>(args: &Bound<'py, PyTuple>) -> PyResult<Vec<(f32, f32)>> {
    let mut points = Vec::with_capacity(args.len());

    for (i, item) in args.iter().enumerate() {
        let tuple = item.downcast::<PyTuple>().map_err(|_| {
            PyValueError::new_err(format!("Item at index {} is not a tuple", i))
        })?;

        if tuple.len() != 2 {
            return Err(PyValueError::new_err(format!(
                "Item at index {} must be a 2-tuple (x, y)", i
            )));
        }

        let x = tuple.get_item(0)?.extract::<f32>().map_err(|_| {
            PyValueError::new_err(format!("x at index {} is not a float", i))
        })?;
        let y = tuple.get_item(1)?.extract::<f32>().map_err(|_| {
            PyValueError::new_err(format!("y at index {} is not a float", i))
        })?;

        points.push((x, y));
    }

    Ok(points)
}
