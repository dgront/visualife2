use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub(crate) mod py_style;
pub use crate::styling::py_style::PyStyle;

pub(crate) mod py_style_manager;
use crate::styling::py_style_manager::PyStyleManager;

pub mod py_palettes;
use crate::styling::py_palettes::{tableau10};

#[pyfunction]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> PyResult<String> {
    Ok(visualife::styling::rgb_to_hex(r, g, b))
}

/// Converts a hex color string (e.g., "#FFA500") to an (R, G, B) tuple.
///
/// Raises:
///     ValueError: If the input is not a valid 7-character hex color.
///
/// Returns:
///     Tuple[int, int, int]: The (R, G, B) components as integers.
#[pyfunction]
pub fn hex_to_rgb(hex: &str) -> PyResult<(u8, u8, u8)> {
    visualife::styling::hex_to_rgb(hex).map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}

#[pyfunction]
pub fn darker(color_hex: &str, fraction: f32) -> PyResult<String> {
    visualife::styling::darker(color_hex, fraction)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}

#[pyfunction]
pub fn lighter(color_hex: &str, fraction: f32) -> PyResult<String> {
    visualife::styling::lighter(color_hex, fraction)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}

/// Mixes two hex colors, shifting `color1` toward `color2` by `(1.0 - fraction)`.
#[pyfunction]
pub fn mix_colors(color1: &str, color2: &str, fraction: f32) -> PyResult<String> {
    if !(0.0..=1.0).contains(&fraction) {
        return Err(PyValueError::new_err("fraction must be between 0.0 and 1.0"));
    }

    match visualife::styling::mix_colors(color1, color2, fraction) {
        Ok(result) => Ok(result),
        Err(msg) => Err(PyValueError::new_err(msg)),
    }
}

pub fn init_submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "styling")?;
    m.add_function(wrap_pyfunction!(rgb_to_hex, m)?)?;
    m.add_function(wrap_pyfunction!(hex_to_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(darker, m)?)?;
    m.add_function(wrap_pyfunction!(lighter, m)?)?;
    m.add_function(wrap_pyfunction!(mix_colors, m)?)?;
    m.add_function(wrap_pyfunction!(tableau10, m)?)?;

    m.add_class::<PyStyle>()?;
    m.add_class::<PyStyleManager>()?;

    parent.add_submodule(m)?;
    Ok(())
}