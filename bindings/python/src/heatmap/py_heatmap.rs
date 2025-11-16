use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PySequence};

use crate::py_svg_drawing::PySvgDrawing;
use visualife::heatmap::Heatmap;

/// Python wrapper for `visualife::heatmap::Heatmap`
#[pyclass(name = "Heatmap")]
pub struct PyHeatmap {
    inner: Heatmap,
}

#[pymethods]
impl PyHeatmap {
    /// Construct from a nested Python sequence (list of lists) of numbers.
    ///
    /// Args:
    ///     id: str|int
    ///     box_width: float
    ///     box_height: float
    ///     data: Sequence[Sequence[float]]
    ///
    /// Raises:
    ///     TypeError/ValueError for malformed inputs.
    #[new]
    #[pyo3(signature = (id, box_width, box_height, data))]
    fn new<'py>(
        id: &Bound<'py, PyAny>,
        box_width: f32,
        box_height: f32,
        data: &Bound<'py, PyAny>,
    ) -> PyResult<Self> {
        let element_id = crate::extract_element_id(id)?;
        let matrix = extract_2d_f64(data)?;
        Ok(Self {
            inner: Heatmap::from_matrix(element_id, box_width, box_height, matrix),
        })
    }

    /// Set the color map explicitly (overrides the default range-based one).
    ///
    /// Expects a `visualife.styling.PyColorMap` instance.
    #[pyo3(signature = (cmap))]
    fn set_colormap(&mut self, cmap: &crate::styling::py_color_map::PyColorMap) {
        self.inner.cmap = cmap.inner.clone();
    }

    /// Number of rows.
    #[getter]
    fn nrows(&self) -> usize {
        self.inner.count_rows()
    }

    /// Number of columns.
    #[getter]
    fn ncols(&self) -> usize {
        self.inner.count_columns()
    }

    /// Optional row labels (if present).
    #[getter]
    fn row_labels(&self) -> Option<Vec<String>> {
        self.inner.row_labels().clone()
    }

    /// Optional column labels (if present).
    #[getter]
    fn col_labels(&self) -> Option<Vec<String>> {
        self.inner.col_labels().clone()
    }

    /// Set row labels (length must equal number of rows).
    #[pyo3(signature = (labels))]
    fn set_row_labels(&mut self, labels: Vec<String>) -> PyResult<()> {
        self.inner
            .set_row_labels(labels)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Set column labels (length must equal number of columns).
    #[pyo3(signature = (labels))]
    fn set_col_labels(&mut self, labels: Vec<String>) -> PyResult<()> {
        self.inner
            .set_col_labels(labels)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Position offsets.
    #[getter]
    fn offset_x(&self) -> f32 {
        self.inner.offset_x
    }
    #[setter]
    fn set_offset_x(&mut self, v: f32) {
        self.inner.offset_x = v;
    }

    #[getter]
    fn offset_y(&self) -> f32 {
        self.inner.offset_y
    }
    #[setter]
    fn set_offset_y(&mut self, v: f32) {
        self.inner.offset_y = v;
    }

    /// Add this heatmap to an existing `SvgDrawing`.
    fn _add_element_to_drawing(&self, drawing: &mut PySvgDrawing) -> PyResult<()> {
        let el = self.inner.create_element();
        drawing.inner.add_element(el);
        Ok(())
    }
}

/// Parse `Sequence[Sequence[number]] -> Vec<Vec<f64>>`
fn extract_2d_f64(ob: &Bound<PyAny>) -> PyResult<Vec<Vec<f64>>> {
    let seq: &Bound<PySequence> = ob
        .cast::<PySequence>()
        .map_err(|_| PyTypeError::new_err("Heatmap data must be a sequence of sequences"))?;

    let outer_len = seq.len()?;
    if outer_len == 0 { return Err(PyValueError::new_err("Heatmap data must not be empty")) }
    let mut out: Vec<Vec<f64>> = Vec::with_capacity(outer_len);
    for i in 0..seq.len()? {
        let row_any = seq.get_item(i)?;

        let row_seq = row_any
            .cast::<PySequence>()
            .map_err(|_| PyTypeError::new_err(format!("Heatmap data row {i} is not a sequence")))?;
        let row_len = row_seq.len()?;
        if row_len == 0 { return Err(PyValueError::new_err(format!("row {i} must not be empty"))) }
        let mut row: Vec<f64> = Vec::with_capacity(row_len);
        for j in 0..row_len {
            let x: f64 = row_seq.get_item(j)?.extract()?;
            row.push(x);
        }
        out.push(row);
    }

    Ok(out)
}
