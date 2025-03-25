pub mod style;
pub mod basic_shapes;
pub mod mindmap;
mod svg_drawing;
pub mod colors;
mod style_manager;
pub use style_manager::StyleManager;

pub use svg_drawing::SvgDrawing;