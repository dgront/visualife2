use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyTuple;

use crate::{PyElementID,extract_element_id};
use crate::ElementType;
use crate::styling::{PyStyle};

use visualife::{SvgDrawing};
use visualife::basic_shapes::{SvgElement, ElementID};

macro_rules! assign_f32_args {
    ($args:ident, $expected:expr, ( $( $var:ident ),+ )) => {
        if $args.len() != $expected {
            return Err(pyo3::exceptions::PyValueError::new_err(
                format!("Expected {} arguments, got {}", $expected, $args.len())
            ));
        }
        let mut i = 0;
        $(
            let $var = $args.get_item(i)?.extract::<f32>()?;
            i += 1;
        )+
    };
}

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

    /// Adds a style to the drawing and returns its style ID.
    pub fn add_style(&mut self, style: &PyStyle) -> u32 {
        self.inner.styles_mut().add_style(style.inner.clone())
    }

    /// Binds a style to an element by ID.
    pub fn style_element(&mut self, style_id: u32, element_id: &PyAny) -> PyResult<()> {
        let id = extract_element_id(element_id)?;
        self.inner.styles_mut().style_element(style_id, &id);
        Ok(())
    }

    /// Draws the SVG to stdout.
    fn draw(&mut self) { self.inner.draw(); }

    /// Adds an element by type and arguments.
    fn add_element(&mut self, element_type: ElementType, id: &PyAny, args: &PyTuple) -> PyResult<()> {

        let id = extract_element_id(id)?;
        let el = match element_type {
            ElementType::Line => Self::parse_line(&id, args)?,
            ElementType::Rect => Self::parse_rect(&id, args)?,
            ElementType::Circle => Self::parse_circle(&id, args)?,
            ElementType::Ellipse => Self::parse_ellipse(&id, args)?,
            ElementType::Polygon => Self::parse_polygon(&id, args)?,
            ElementType::Polyline => Self::parse_polyline(&id, args)?,
            ElementType::Path => Self::parse_path(&id, args)?,
            ElementType::Text => Self::parse_text(&id, args)?,
            ElementType::Group => Self::parse_group(&id)?,
        };
        self.inner.add_element(el);
        Ok(())
    }

    /// Adds a new SVG element to a group identified by group_id.
    fn add_element_to_group( &mut self, group_id: &PyAny,
        element_type: ElementType, id: &PyAny, args: &PyTuple, ) -> PyResult<()> {

        let id = extract_element_id(id)?;
        let group_id = extract_element_id(group_id)?;
        let el = match element_type {
            ElementType::Line => Self::parse_line(&id, args)?,
            ElementType::Rect => Self::parse_rect(&id, args)?,
            ElementType::Circle => Self::parse_circle(&id, args)?,
            ElementType::Ellipse => Self::parse_ellipse(&id, args)?,
            ElementType::Polygon => Self::parse_polygon(&id, args)?,
            ElementType::Polyline => Self::parse_polyline(&id, args)?,
            ElementType::Path => Self::parse_path(&id, args)?,
            ElementType::Text => Self::parse_text(&id, args)?,
            ElementType::Group => return Err(PyValueError::new_err("Cannot nest a new group this way")),
        };

        self.inner
            .add_element_to_group(el, group_id.clone())
            .map_err(PyValueError::new_err)
    }
}

impl PySvgDrawing {

    fn parse_line(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        assign_f32_args!(args, 4, (x1, y1, x2, y2));
        Ok(SvgElement::Line { id: id.clone(), x1, y1, x2, y2 })
    }

    fn parse_circle(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        assign_f32_args!(args, 3, (cx, cy, r));
        Ok(SvgElement::Circle { id: id.clone(), cx, cy, r })
    }

    fn parse_rect(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        assign_f32_args!(args, 4, (x, y, width, height));
        Ok(SvgElement::Rect { id: id.clone(), x, y, width, height })
    }

    fn parse_ellipse(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        assign_f32_args!(args, 4, (cx, cy, rx, ry));
        Ok(SvgElement::Ellipse { id: id.clone(), cx, cy, rx, ry })
    }

    fn parse_polygon(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        let points = PySvgDrawing::extract_point_list(args)?;
        Ok(SvgElement::Polygon { id: id.clone(), points })
    }

    fn parse_polyline(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        let points = PySvgDrawing::extract_point_list(args)?;
        Ok(SvgElement::Polyline { id: id.clone(), points })
    }

    fn parse_group(id: &ElementID) -> PyResult<SvgElement> {

        Ok(SvgElement::Group { id: id.clone(), elements: Vec::new(), })
    }

    fn parse_text(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        if args.len() != 3 {
            return Err(PyValueError::new_err("Text requires 3 arguments: x, y, content"));
        }

        let x = args.get_item(0)?.extract::<f32>()?;
        let y = args.get_item(1)?.extract::<f32>()?;
        let content = args.get_item(2)?.extract::<String>()?;

        Ok(SvgElement::Text { id: id.clone(), x, y, content, })
    }

    fn parse_path(id: &ElementID, args: &PyTuple) -> PyResult<SvgElement> {
        if args.len() != 1 {
            return Err(PyValueError::new_err("Path requires 1 argument: d (string)"));
        }
        let d = args.get_item(0)?.extract::<String>()?;
        Ok(SvgElement::Path { id: id.clone(), d })
    }

    /// Extracts a list of (f32, f32) pairs from a PyTuple.
    /// Each item must be a 2-element tuple of floats.
    fn extract_point_list(args: &PyTuple) -> PyResult<Vec<(f32, f32)>> {
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
}


