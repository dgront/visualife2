//! Defines all the basic shapes that can be drawn in an SVG file, such as circles or paths.

mod elements;
pub use elements::*;

mod path_builder;
pub use path_builder::*;

mod attributes;
pub use attributes::*;

mod shapes;
pub use shapes::*;

mod fonts;
pub use fonts::*;
