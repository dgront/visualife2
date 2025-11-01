mod style_manager;
pub use style_manager::StyleManager;

mod colors;
pub use colors::*;

mod style;
pub use style::*;

mod errors;
pub use errors::*;

/// Provides several predefined color palettes for visualization.
///
/// This module contains several most popular color palettes as ``\[&str;\]`` arrays.
///
/// # Available categorical palettes
///
/// Tableau 10 palette:
#[doc = include_str!("../../tests/expected_drawings/styling/tableau10.svg")]
///
/// Accent palette:
#[doc = include_str!("../../tests/expected_drawings/styling/accent.svg")]
///
/// Categorical Accent palette:
#[doc = include_str!("../../tests/expected_drawings/styling/categorical_accent.svg")]
///
/// Paired palette:
#[doc = include_str!("../../tests/expected_drawings/styling/paired.svg")]
///
/// Pastel palette:
#[doc = include_str!("../../tests/expected_drawings/styling/pastel.svg")]
///
/// Tableau 20 palette:
#[doc = include_str!("../../tests/expected_drawings/styling/tableau20.svg")]
///
/// Viridis palette:
#[doc = include_str!("../../tests/expected_drawings/styling/viridis.svg")]
///
/// Okabe-Ito palette:
#[doc = include_str!("../../tests/expected_drawings/styling/okabe_ito.svg")]
///
/// ggplot2 Default palette:
#[doc = include_str!("../../tests/expected_drawings/styling/ggplot2_default.svg")]
///
/// IBM Colors palette:
#[doc = include_str!("../../tests/expected_drawings/styling/ibm_colors.svg")]
///
/// ColorBrewer Set1 palette:
#[doc = include_str!("../../tests/expected_drawings/styling/colorbrewer_set1.svg")]
///
/// Plotly10 palette:
#[doc = include_str!("../../tests/expected_drawings/styling/plotly10.svg")]
///
/// Dark palette:
#[doc = include_str!("../../tests/expected_drawings/styling/dark.svg")]
///
/// Dark2 palette:
#[doc = include_str!("../../tests/expected_drawings/styling/dark2.svg")]


/// # Examples
///
/// ```rust
/// use visualife::basic_shapes::SvgElement;
/// use visualife::styling::{Style, StyleManager};
/// use visualife::styling::palettes::TABLEAU10;
/// use visualife::SvgDrawing;
///
/// let n_colors = TABLEAU10.len();
/// let mut drawing = SvgDrawing::new(130.0, 30.0);
/// for i in 0..TABLEAU10.len() {
///     let r = SvgElement::rect("r1", (i+12 + 5) as f32, 5.0, 10.0, 20.0)
///         .with_style(Style::new().fill(TABLEAU10[i]));
///     drawing.add_element(r);
/// }
/// ```
pub mod palettes;

mod color_map;
pub use color_map::*;
