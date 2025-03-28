use pyo3::prelude::*;

use visualife::styling::palettes::*;

/// Returns the Tableau 10 color palette as a list of hex strings.
#[pyfunction]
pub fn tableau10() -> Vec<String> {
    TABLEAU10.iter().map(|s| s.to_string()).collect()
}