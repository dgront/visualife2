
/// Defines all the basic shapes that can be drawn in an SVG file, such as circles or paths.
pub mod basic_shapes;
/// Library for drawing mindmaps, inspired by the [TikZ MindMap](https://tikz.dev/library-mindmaps) library.
pub mod mindmap;
pub mod styling;

mod svg_drawing;
pub use svg_drawing::SvgDrawing;