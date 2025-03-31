
/// Enum representing available SVG elements with associated geometric and style information.
mod internal {
    use crate::{ElementID, SvgDrawing};
    use crate::styling::{Style, StyleManager};
    #[derive(Clone)]
    pub enum SvgElement {
        // Represents an SVG line element, defined by its start (x1, y1) and end (x2, y2) points.
        Line { id: ElementID, x1: f32, y1: f32, x2: f32, y2: f32 },

        // Represents an SVG rectangle element, with its position, width, and height.
        Rect { id: ElementID, x: f32, y: f32, width: f32, height: f32 },

        // Represents an SVG circle element, defined by its center (cx, cy) and radius `r`.
        Circle { id: ElementID, cx: f32, cy: f32, r: f32 },

        // Represents an SVG ellipse element, defined by its center (cx, cy) and radii (rx, ry).
        Ellipse { id: ElementID, cx: f32, cy: f32, rx: f32, ry: f32 },

        // Represents an SVG polygon element, defined by a list of 2D points.
        Polygon { id: ElementID, points: Vec<(f32, f32)> },

        // Represents an SVG polyline element, similar to a polygon but not closed.
        Polyline { id: ElementID, points: Vec<(f32, f32)> },

        // Represents an SVG path element, defined by a `d` attribute (path data string).
        Path { id: ElementID, d: String },

        // Represents an SVG text element, placed at (x, y) with the specified string content.
        Text { id: ElementID, x: f32, y: f32, content: String },

        // Represents an SVG group element, which can contain multiple child elements.
        Group { id: ElementID, elements: Vec<SvgElement> },
    }

    impl SvgElement {
        /// Creates a rectangle element with the given ID, position, and size.
        pub fn rect(id: impl Into<ElementID>, x: f32, y: f32, width: f32, height: f32) -> Self {
            Self::Rect { id: id.into(), x, y, width, height }
        }

        /// Creates a line element from (x1, y1) to (x2, y2).
        pub fn line(id: impl Into<ElementID>, x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
            Self::Line { id: id.into(), x1, y1, x2, y2 }
        }

        /// Creates a circle element with the given center and radius.
        pub fn circle(id: impl Into<ElementID>, cx: f32, cy: f32, r: f32) -> Self {
            Self::Circle { id: id.into(), cx, cy, r }
        }

        /// Creates an ellipse element with the given center and radii.
        pub fn ellipse(id: impl Into<ElementID>, cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
            Self::Ellipse { id: id.into(), cx, cy, rx, ry }
        }

        /// Creates a polygon element from a list of (x, y) points.
        pub fn polygon(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
            Self::Polygon { id: id.into(), points }
        }

        /// Creates a polyline element from a list of (x, y) points (not closed).
        pub fn polyline(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
            Self::Polyline { id: id.into(), points }
        }

        /// Creates a path element with the given SVG path data string (`d` attribute).
        pub fn path(id: impl Into<ElementID>, d: impl Into<String>) -> Self {
            Self::Path { id: id.into(), d: d.into() }
        }

        /// Creates a text element at position (x, y) with the specified content.
        pub fn text(id: impl Into<ElementID>, x: f32, y: f32, content: impl Into<String>) -> Self {
            Self::Text { id: id.into(), x, y, content: content.into() }
        }

        /// Creates a group element containing nested elements.
        pub fn group(id: impl Into<ElementID>, elements: Vec<SvgElement>) -> Self {
            Self::Group { id: id.into(), elements }
        }

        /// Registers the style with the drawing and applies it to this element.
        ///
        /// Returns the style ID so it can be reused.
        ///
        /// # Examples
        /// ```
        /// use visualife::basic_shapes::SvgElement;
        /// use visualife::styling::Style;
        /// use visualife::SvgDrawing;
        /// let mut drawing = SvgDrawing::new(100.0, 100.0);
        /// let style = Style::new().fill("#a6cee3").stroke("#1f78b4");
        /// let  style_id = SvgElement::circle("circle1", 25.0, 25.0, 20.0).with_style(&mut drawing, style);
        /// drawing.add_element(SvgElement::circle("circle2", 75.0, 75.0, 20.0));
        /// drawing.style_element(style_id, "circle2");
        /// ```
        pub fn with_style(&self, drawing: &mut SvgDrawing, style: Style) -> u32 {
            let style_id = drawing.styles_mut().add_style(style);
            drawing.styles_mut().style_element(style_id, self.id().clone());
            style_id
        }

        /// Returns a reference to the element's ID.
        pub fn id(&self) -> &ElementID {
            match self {
                SvgElement::Line { id, .. } => id,
                SvgElement::Rect { id, .. } => id,
                SvgElement::Circle { id, .. } => id,
                SvgElement::Ellipse { id, .. } => id,
                SvgElement::Polygon { id, .. } => id,
                SvgElement::Polyline { id, .. } => id,
                SvgElement::Path { id, .. } => id,
                SvgElement::Text { id, .. } => id,
                SvgElement::Group { id, .. } => id,
            }
        }

        pub fn to_svg(&self, style_mgr: &StyleManager) -> String {
            let style = style_mgr.get_style(self.id());
            let style_str = style.map_or_else(String::new, |s| s.to_svg());
            match self {
                SvgElement::Line { id, x1, y1, x2, y2 } => format!("<line id=\"{}\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" {}/>", id, x1, y1, x2, y2, style_str),
                SvgElement::Rect { id, x, y, width, height } => format!("<rect id=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" {}/>", id, x, y, width, height, style_str),
                SvgElement::Circle { id, cx, cy, r } => format!("<circle id=\"{}\" cx=\"{}\" cy=\"{}\" r=\"{}\" {}/>", id, cx, cy, r, style_str),
                SvgElement::Ellipse { id, cx, cy, rx, ry } => format!("<ellipse id=\"{}\" cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" {}/>", id, cx, cy, rx, ry, style_str),
                SvgElement::Polygon { points, .. } => {
                    let points_str = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<String>>().join(" ");
                    format!(r#"<polygon points="{}"{} />"#, points_str, style_str)
                },
                SvgElement::Polyline { points, .. } => {
                    let points_str = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<String>>().join(" ");
                    format!(r#"<polyline points="{}"{} />"#, points_str, style_str)
                },
                SvgElement::Path { id, d, .. } => format!(r#"<path id="{}" d="{}"{} />"#, id, d, style_str),
                SvgElement::Text { x, y, content, .. } => format!(r#"<text x="{}" y="{}"{}>{}</text>"#, x, y, style_str, content),
                SvgElement::Group { id, elements } => {
                    let inner_svg = elements
                        .iter()
                        .map(|el| el.to_svg(style_mgr))
                        .collect::<Vec<String>>()
                        .join("\n\t");

                    format!("<g id=\"{}\" {}>\n\t{}\n</g>", id, style_str, inner_svg)
                }
            }
        }
    }
}
// ✅ Expose only the type and its constructor API (not the variants)
pub use internal::SvgElement;

