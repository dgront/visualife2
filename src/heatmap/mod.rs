//! # Visualises 2D data as a heat map
//!
//! Represents two-dimensional numerical data as a rectangular  heat map,
//! assigning a color to each value from a given color scale / palette.
//!
#![doc = include_str!("../../tests/expected_drawings/heatmap/cities.svg")]
//!
mod heatmap;
pub use heatmap::*;

mod errors;
pub use errors::*;
