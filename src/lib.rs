//! # Versatile library for SVG drawing in Rust
//!

/// Defines all the basic shapes that can be drawn in an SVG file, such as circles or paths.
pub mod basic_shapes;

pub mod mindmap;
/// Visualize array-like data as a heatmap
pub mod heatmap;
/// Define styles for SVG elements.
pub mod styling;

mod element_id;

pub use element_id::*;

mod svg_drawing;
pub use svg_drawing::SvgDrawing;

mod errors;
pub use errors::*;

/// Normalizes whitespace in a string, replacing all whitespace character blocks with a single space.
/// This function is used to compare SVG as strings in unit tests.
pub(crate) fn normalize_whitespace(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let cleaned = line
                .trim_end()
                .split(|c| c == ' ' || c == '\t')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            // Remove any space directly before '>'
            cleaned.replace(" >", ">")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
