//! # Library for drawing mindmaps, inspired by the [TikZ MindMap](https://tikz.dev/library-mindmaps) library.
//!
//! A mindmap is a visual diagram that organizes information around a central concept
//! using branches to represent related ideas, relationships, or hierarchical structures.
//!
//! ## Create an empty [`Mindmap`](crate::mindmap::Mindmap)
//! ```
//! use visualife::mindmap::Mindmap;
//! let mut mndmp = Mindmap::new("a_mindmap", 50.0);
//! ```
//! `50.0` is the default node radius, `"a_mindmap"` is the unique ID assigned to corresponding SVG group.
//!
//! ## Add some nodes
//! ```
//! # use visualife::mindmap::Mindmap;
//! # use visualife::Point;
//! # let mut mndmp = Mindmap::new("a_mindmap", 50.0);
//! let root_node = mndmp.place_node("root", "Root", Point::new(120.0, 80.0));
//! ```
//!
//! You may also grow nodes into a desired direction by specifying the angle:
//! ```
//! # use visualife::mindmap::Mindmap;
//! # use visualife::Point;
//! # let mut mndmp = Mindmap::new("a_mindmap", 50.0);
//! # let root_node_id = mndmp.place_node("root", "Root", Point::new(120.0, 80.0)).id.clone();
//! let n_new_nodes = 5;
//! for i in 0..n_new_nodes {
//!     let angle = (90.0 / ((n_new_nodes - 1) as f32) * i as f32);
//!     let new_node = mndmp.grow_node(&format!("n:{i}"),&format!("{angle}°"), angle, root_node_id.clone());
//! }
//! ```
//! You may want to change how a [`Node`](crate::mindmap::Node) looks like by adding a style to it:
//! ```
//! # use visualife::mindmap::Mindmap;
//! use visualife::styling::{Style, darker};
//! # fn main() -> Result<(), anyhow::Error> {
//! # let mut mndmp = Mindmap::new("a_mindmap", 50.0);
//! # let root_node_id = mndmp.place_node("root", "Root", (120.0, 80.0).into()).id.clone();
//! # let n_new_nodes = 5;
//! let mut fill = String::from("#FFFFFF");
//! for i in 0..n_new_nodes {
//! #     let angle = (90.0 / ((n_new_nodes - 1) as f32) * i as f32);
//! #     let new_node = mndmp.grow_node(&format!("n:{i}"),&format!("{angle}°"), angle, root_node_id.clone());
//!     fill = darker(fill.as_str(), 0.1)?;
//!     let style = Style::new()
//!         .fill(fill.as_str())
//!         .stroke_dasharray([15.0, 5.0])
//!         .stroke_width(3.0)
//!         .stroke("black");
//!     new_node.with_style(style);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Create [`SvgDrawing`](crate::SvgDrawing) and add the [`Mindmap`]
//! ```
//! # fn main() -> Result<(), anyhow::Error> {
//! use visualife::SvgDrawing;
//! # use std::fs;
//! # use visualife::mindmap::Mindmap;
//! # let mut mndmp = Mindmap::new("a_mindmap", 50.0);
//! let mut drawing = SvgDrawing::new(300.0, 300.0);
//! drawing.add_element(mndmp.create_element());
//! drawing.save_svg("figure.svg")?;
//! # fs::remove_file("figure.svg").unwrap(); // cleanup
//! # Ok(())
//! # }
//! ```
//!
#![doc = include_str!("../../tests/expected_drawings/mindmap/grow_nodes.svg")]
//!

mod connector;
mod node;
pub use node::Node;

mod errors;
pub use errors::*;

mod mindmap;
pub use mindmap::{Mindmap};
use crate::Point;

const FOOT_LENGTH_R_FRACTION: f32 = 1.5;    /// foot is 1.5 times the radius

/// Convert Cartesian coordinates of a given ``Point`` to polar coordinates centered at ``origin`` point.
///
/// Returned angle is in radians.
pub fn cartesian_to_polar(origin: &Point, point: &Point) -> (f32, f32) {
    let dx = point.x - origin.x;
    let dy = point.y - origin.y;

    // Calculate the radius
    let r = (dx * dx + dy * dy).sqrt();

    // Calculate the angle in radians
    let theta = dy.atan2(dx);

    (r, theta)
}

/// Convert polar coordinates to cartesian coordinates.
///
/// Returns the x, y coordinates of the ``(radius, angle_rad)`` radial point in the
/// system centered at ``origin``
/// # Example
///
/// ```
/// use visualife::mindmap::polar_to_cartesian;
/// use visualife::Point;
/// let (x, y) = polar_to_cartesian(1.0, (45.0_f32).to_radians(), &Point::default());
/// // Expected values: x = y ≈ √2 / 2 ≈ 0.7071
/// assert!((x - 0.7071).abs() < 1e-4);
/// assert!((y - 0.7071).abs() < 1e-4);
/// let (x, y) = polar_to_cartesian(1.0, (360.0_f32 - 45.0_f32).to_radians(), &Point::default());
/// assert!((x - 0.7071).abs() < 1e-4);
/// assert!((y + 0.7071).abs() < 1e-4);
/// ```
pub fn polar_to_cartesian(radius: f32, angle_rad: f32, origin: &Point) -> (f32, f32) {
    (radius * angle_rad.cos() + origin.x, radius * angle_rad.sin() + origin.y)
}
