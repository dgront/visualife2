//! # Versatile library for SVG drawing in Rust
//!
//! ## How to contribute
#![doc = r#"
Check the [design notes](crate::design_notes) to see the rationale behind the VisuaLife design
"#]

// Expose a docs-only page backed by your Markdown file
pub mod design_notes {
    #![doc = include_str!("../design_notes.md")]
}

// /// Defines all the basic shapes that can be drawn in an SVG file, such as circles or paths.
pub mod basic_shapes;

pub mod mindmap;
/// Visualize array-like data as a heatmap
pub mod heatmap;
/// Define styles for SVG elements.
pub mod styling;

/// Plotting library
pub mod plots;

mod element_id;

pub use element_id::*;

mod svg_drawing;
pub use svg_drawing::SvgDrawing;

mod errors;
mod utils;

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
