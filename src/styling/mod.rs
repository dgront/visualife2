mod style_manager;
pub use style_manager::StyleManager;

mod colors;
pub use colors::*;

mod style;
pub use style::*;

/// Provides several predefined color palettes for visualization.
///
/// This module exposes several commonly used palettes as string constants.
///
/// # Example
/// ```
/// use visualife::styling::{Style, StyleManager};
/// use visualife::basic_shapes::SvgElement::Rect;
/// ```
pub mod palettes;

mod color_map;
pub use color_map::*;
