use pyo3::prelude::*;

use visualife::styling::Style;


#[pyfunction]
fn rgb_to_hex(r: u8, g: u8, b: u8) -> PyResult<String> {
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
fn hex_to_rgb(hex: &str) -> PyResult<(u8, u8, u8)> {
    visualife::styling::hex_to_rgb(hex).map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}

#[pyfunction]
fn darker(color_hex: &str, fraction: f32) -> PyResult<String> {
    visualife::styling::darker(color_hex, fraction)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}

#[pyfunction]
fn lighter(color_hex: &str, fraction: f32) -> PyResult<String> {
    visualife::styling::lighter(color_hex, fraction)
        .map_err(|msg| PyErr::new::<pyo3::exceptions::PyValueError, _>(msg.to_string()))
}


pub fn init_submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let m = PyModule::new(py, "styling")?;
    m.add_function(wrap_pyfunction!(rgb_to_hex, m)?)?;
    m.add_function(wrap_pyfunction!(hex_to_rgb, m)?)?;
    m.add_function(wrap_pyfunction!(darker, m)?)?;
    m.add_function(wrap_pyfunction!(lighter, m)?)?;

    m.add_class::<PyStyle>()?;

    parent.add_submodule(m)?;
    Ok(())
}