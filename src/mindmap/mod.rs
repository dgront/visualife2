mod connector;
mod node;
pub use node::Node;

mod errors;
pub use errors::*;

mod mindmap;
pub use mindmap::{Mindmap};

const FOOT_LENGTH_R_FRACTION: f32 = 1.5;    /// foot is 1.5 times the radius

/// Convert Cartesian coordinates of ``(px,py)`` point to polar coordinates centered at ``(cx,cy)`` point.
///
/// Returned angle is in radians.
pub fn cartesian_to_polar(cx: f32, cy: f32, px: f32, py: f32) -> (f32, f32) {
    let dx = px - cx;
    let dy = py - cy;

    // Calculate the radius
    let r = (dx * dx + dy * dy).sqrt();

    // Calculate the angle in radians
    let theta = dy.atan2(dx);

    (r, theta)
}

/// Convert polar coordinates to cartesian coordinates.
///
/// Returns the x, y coordinates of the ``(radius, angle_rad)`` radial point.
/// # Example
///
/// ```
/// use visualife::mindmap::polar_to_cartesian;
/// let (x, y) = polar_to_cartesian(1.0, (45.0_f32).to_radians(), 0.0, 0.0);
/// // Expected values: x = y ≈ √2 / 2 ≈ 0.7071
/// assert!((x - 0.7071).abs() < 1e-4);
/// assert!((y - 0.7071).abs() < 1e-4);
/// let (x, y) = polar_to_cartesian(1.0, (360.0_f32 - 45.0_f32).to_radians(), 0.0, 0.0);
/// assert!((x - 0.7071).abs() < 1e-4);
/// assert!((y + 0.7071).abs() < 1e-4);
/// ```
pub fn polar_to_cartesian(radius: f32, angle_rad: f32, cx: f32, cy: f32) -> (f32, f32) {
    (radius * angle_rad.cos() + cx, radius * angle_rad.sin() + cy)
}
