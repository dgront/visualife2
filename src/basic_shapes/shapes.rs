use crate::basic_shapes::SvgElement;
use crate::{ElementID, Point};

/// Returns the three vertices of a triangular arrow head.
///
/// The triangle is defined by:
/// - tip at `tip`
/// - base centered at `base`
/// - base width `base_width`, perpendicular to the arrow direction
///
/// The function is geometry-only and does not assume any rendering backend.
///
/// # Examples
/// ```
/// use visualife::basic_shapes::triangle_arrow;
/// use visualife::Point;
///
/// let tri = triangle_arrow("tri", Point::new(0.0, 0.0), Point::new(1.0, 0.0), 0.2);
/// ```
pub fn triangle_arrow(id: impl Into<ElementID>, base: Point, tip: Point, base_width: f32) -> SvgElement {
    // Direction vector from base to tip
    let dx = tip.x - base.x;
    let dy = tip.y - base.y;

    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 {
        return SvgElement::polygon(id.into(), vec![base.into(), base.into(), tip.into()]);
    } else {
        // Unit perpendicular vector
        let px = -dy / len;
        let py = dx / len;

        let half_w = base_width * 0.5;

        // Two base corners
        let p1 = Point::new(base.x + px * half_w, base.y + py * half_w);

        let p2 = Point::new(base.x - px * half_w, base.y - py * half_w);

        SvgElement::polygon(id.into(), vec![p1.into(), p2.into(), tip.into()])
    }
}

/// Create lines that forms a rectangular grid.
///
/// # Example
/// ```
/// use visualife::basic_shapes::grid_lines;
/// use visualife::plots::linspace;
/// use visualife::styling::Style;
/// use visualife::SvgDrawing;
/// let xy = linspace(7, 10.0, 70.0, false);
/// let mut drawing = SvgDrawing::new(80.0, 80.0);
/// let grid = grid_lines("grid", &xy, &xy, true).with_style(
///     Style::new().stroke("#000000").stroke_width(0.1));
/// drawing.add_element(grid);
/// drawing.save_svg("grid.svg").unwrap()
/// ```
pub fn grid_lines(id: impl Into<ElementID>, x: &[f32], y: &[f32], draw_borderlines: bool) -> SvgElement {
    // Nothing to draw if we cannot span both directions.
    if x.len() < 2 || y.len() < 2 {
        return SvgElement::group(id, Vec::new());
    }

    let x_min = x[0];
    let x_max = x[x.len() - 1];
    let y_min = y[0];
    let y_max = y[y.len() - 1];

    // Decide which indices to include.
    let (x_start, x_end_excl) = if draw_borderlines { (0, x.len()) } else { (1, x.len() - 1) };
    let (y_start, y_end_excl) = if draw_borderlines { (0, y.len()) } else { (1, y.len() - 1) };

    let mut elements = Vec::new();

    let id = id.into();
    // Vertical lines at each x, spanning full y-range.
    if x_end_excl > x_start {
        for (i, &xi) in x[x_start..x_end_excl].iter().enumerate() {
            let line_id = format!("{id}-v-{i}");
            elements.push(SvgElement::line(&line_id, xi, y_min, xi, y_max));
        }
    }

    // Horizontal lines at each y, spanning full x-range.
    if y_end_excl > y_start {
        for (j, &yj) in y[y_start..y_end_excl].iter().enumerate() {
            let line_id = format!("{id}-h-{j}");
            elements.push(SvgElement::line(&line_id, x_min, yj, x_max, yj));
        }
    }

    SvgElement::group(id, elements)
}
