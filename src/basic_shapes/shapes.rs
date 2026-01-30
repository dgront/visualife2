use crate::basic_shapes::SvgElement;
use crate::ElementID;

/// Returns the three vertices of a triangular arrow head.
///
/// The triangle is defined by:
/// - tip at `(x_tip, y_tip)`
/// - base centered at `(x_base, y_base)`
/// - base width `base_width`, perpendicular to the arrow direction
///
/// The function is geometry-only and does not assume any rendering backend.
///
/// # Examples
/// ```
/// use visualife::basic_shapes::triangle_arrow;
/// let tri = triangle_arrow("tri", 0.0, 0.0, 1.0, 0.0, 0.2);
/// ```
pub fn triangle_arrow(id: impl Into<ElementID>, x_base: f32, y_base: f32, x_tip: f32, y_tip: f32, base_width: f32) -> SvgElement {
    // Direction vector from base to tip
    let dx = x_tip - x_base;
    let dy = y_tip - y_base;

    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return SvgElement::polygon(id.into(),vec![(x_base, y_base), (x_base, y_base), (x_tip, y_tip)]);
    } else {
        // Unit perpendicular vector
        let px = -dy / len;
        let py = dx / len;

        let half_w = base_width * 0.5;

        // Two base corners
        let x1 = x_base + px * half_w;
        let y1 = y_base + py * half_w;

        let x2 = x_base - px * half_w;
        let y2 = y_base - py * half_w;

        SvgElement::polygon(id.into(), vec![(x1, y1), (x2, y2), (x_tip, y_tip)])
    }
}