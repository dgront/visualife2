use pyo3::prelude::*;

use visualife::styling::palettes::*;

/// Returns the Tableau 10 color palette as a list of hex strings.
#[pyfunction]
pub fn tableau10() -> Vec<String> { TABLEAU10.iter().map(|s| s.to_string()).collect() }

/// Returns the "Categorical Accent" palettes used in modern data visualization tools, as a list of hex strings.
#[pyfunction]
pub fn categorical_accent() -> Vec<String> { CATEGORICAL_ACCENT.iter().map(|s| s.to_string()).collect() }

/// Returns the Tableau 20 color palette as a list of hex strings.
#[pyfunction]
pub fn tableau20() -> Vec<String> { TABLEAU20.iter().map(|s| s.to_string()).collect() }

/// Returns the Viridis color palette as a list of hex strings.
#[pyfunction]
pub fn viridis() -> Vec<String> { VIRIDIS.iter().map(|s| s.to_string()).collect() }

/// Returns the Pastel color palette as a list of hex strings.
#[pyfunction]
pub fn pastel() -> Vec<String> { PASTEL.iter().map(|s| s.to_string()).collect() }

/// Returns the Accent color palette as a list of hex strings.
#[pyfunction]
pub fn accent() -> Vec<String> { ACCENT.iter().map(|s| s.to_string()).collect() }

/// Returns the Paired color palette as a list of hex strings.
#[pyfunction]
pub fn paired() -> Vec<String> { PAIRED.iter().map(|s| s.to_string()).collect() }

/// Returns the Red-Blue diverging palette as a list of hex strings.
#[pyfunction]
pub fn red_blue() -> Vec<String> { RED_BLUE.iter().map(|s| s.to_string()).collect() }