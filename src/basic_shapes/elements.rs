use crate::{ElementID};
use crate::basic_shapes::SvgAttributes;
use crate::styling::{Style};

/// Represents an SVG graphical element.
///
/// `SvgElement` is a public wrapper around an internal enum that defines geometric primitives
/// used in an SVG drawing, such as lines, rectangles, circles, ellipses, text, and groups.
/// Each element stores its geometry, unique ID, and shared attributes (style, transform, mask)
/// via [`SvgAttributes`].
///
/// Elements are created using factory methods such as:
/// - [`SvgElement::circle`]
/// - [`SvgElement::rect`]
/// - [`SvgElement::line`]
///
/// Style-related attributes can be added fluently using:
/// - [`SvgElement::with_style`]
/// - [`SvgElement::with_transform`]
/// - [`SvgElement::with_mask`]
///
/// # Example
/// ```rust
/// use visualife::basic_shapes::SvgElement;
/// use visualife::styling::Style;
///
/// let circle = SvgElement::circle("node1", 50.0, 50.0, 20.0)
///     .with_style(Style::new().fill("skyblue").stroke("navy"))
///     .with_transform("rotate(45, 50, 50)")
///     .with_mask("myMask");
///
/// let rect = SvgElement::rect("box", 10.0, 10.0, 80.0, 50.0);
/// ```
#[derive(Clone)]
pub struct SvgElement {
    inner: SvgElementKind,
}

#[derive(Clone)]
enum SvgElementKind {
    /// Represents an SVG line element, defined by its start (x1, y1) and end (x2, y2) points.
    Line { id: ElementID, x1: f32, y1: f32, x2: f32, y2: f32, attr: SvgAttributes },

    /// Represents an SVG rectangle element, with its position, width, and height.
    Rect { id: ElementID, x: f32, y: f32, width: f32, height: f32, attr: SvgAttributes },

    /// Represents an SVG circle element, defined by its center (cx, cy) and radius `r`.
    Circle { id: ElementID, cx: f32, cy: f32, r: f32, attr: SvgAttributes },

    /// Represents an SVG ellipse element, defined by its center (cx, cy) and radii (rx, ry).
    Ellipse { id: ElementID, cx: f32, cy: f32, rx: f32, ry: f32, attr: SvgAttributes },

    /// Represents an SVG polygon element, defined by a list of 2D points.
    Polygon { id: ElementID, points: Vec<(f32, f32)>, attr: SvgAttributes },

    /// Represents an SVG polyline element, similar to a polygon but not closed.
    Polyline { id: ElementID, points: Vec<(f32, f32)>, attr: SvgAttributes },

    /// Represents an SVG path element, defined by a `d` attribute (path data string).
    Path { id: ElementID, d: String, attr: SvgAttributes },

    /// Represents an SVG text element, placed at (x, y) with the specified string content.
    Text { id: ElementID, x: f32, y: f32, content: String, attr: SvgAttributes },

    /// Represents an SVG group element, which can contain multiple child elements.
    Group { id: ElementID, elements: Vec<SvgElement>, attr: SvgAttributes },
}

// === Public API ===

impl SvgElement {

    pub fn line(id: impl Into<ElementID>, x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { inner: SvgElementKind::Line { id: id.into(), x1, y1, x2, y2, attr: SvgAttributes::default() } }
    }

    pub fn rect(id: impl Into<ElementID>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { inner: SvgElementKind::Rect { id: id.into(), x, y, width, height, attr: SvgAttributes::default() } }
    }

    pub fn circle(id: impl Into<ElementID>, cx: f32, cy: f32, r: f32) -> Self {
        Self { inner: SvgElementKind::Circle { id: id.into(), cx, cy, r, attr: SvgAttributes::default() } }
    }

    pub fn ellipse(id: impl Into<ElementID>, cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        Self { inner: SvgElementKind::Ellipse { id: id.into(), cx, cy, rx, ry, attr: SvgAttributes::default() } }
    }

    pub fn polygon(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
        Self { inner: SvgElementKind::Polygon { id: id.into(), points, attr: SvgAttributes::default() } }
    }

    pub fn polyline(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
        Self { inner: SvgElementKind::Polyline { id: id.into(), points, attr: SvgAttributes::default() } }
    }

    pub fn path(id: impl Into<ElementID>, d: impl Into<String>) -> Self {
        Self { inner: SvgElementKind::Path { id: id.into(), d: d.into(), attr: SvgAttributes::default() } }
    }

    pub fn text(id: impl Into<ElementID>, x: f32, y: f32, content: impl Into<String>) -> Self {
        Self { inner: SvgElementKind::Text { id: id.into(), x, y, content: content.into(), attr: SvgAttributes::default() } }
    }

    pub fn group(id: impl Into<ElementID>, elements: Vec<SvgElement>) -> Self {
        Self { inner: SvgElementKind::Group { id: id.into(), elements, attr: SvgAttributes::default() } }
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.attr_mut().style = Some(style);
        self
    }

    pub fn with_transform(mut self, transform: impl Into<String>) -> Self {
        self.attr_mut().transform = Some(transform.into());
        self
    }

    pub fn with_mask(mut self, mask_id: impl Into<String>) -> Self {
        self.attr_mut().mask = Some(mask_id.into());
        self
    }

    pub fn id(&self) -> &ElementID {
        match &self.inner {
            SvgElementKind::Line { id, .. }
            | SvgElementKind::Rect { id, .. }
            | SvgElementKind::Circle { id, .. }
            | SvgElementKind::Ellipse { id, .. }
            | SvgElementKind::Polygon { id, .. }
            | SvgElementKind::Polyline { id, .. }
            | SvgElementKind::Path { id, .. }
            | SvgElementKind::Text { id, .. }
            | SvgElementKind::Group { id, .. } => id,
        }
    }

    /// Converts the element into an SVG XML fragment using the provided style manager.
    pub fn to_svg(&self) -> String {

        let attr_str = self.attr().to_svg_fragment();

        match &self.inner {
            SvgElementKind::Line { id, x1, y1, x2, y2, .. } =>
                format!(r#"<line id="{}" x1="{}" y1="{}" x2="{}" y2="{}"{} />"#, id, x1, y1, x2, y2, attr_str),

            SvgElementKind::Rect { id, x, y, width, height, .. } =>
                format!(r#"<rect id="{}" x="{}" y="{}" width="{}" height="{}"{} />"#, id, x, y, width, height, attr_str),

            SvgElementKind::Circle { id, cx, cy, r, .. } =>
                format!(r#"<circle id="{}" cx="{}" cy="{}" r="{}"{} />"#, id, cx, cy, r, attr_str),

            SvgElementKind::Ellipse { id, cx, cy, rx, ry, .. } =>
                format!(r#"<ellipse id="{}" cx="{}" cy="{}" rx="{}" ry="{}"{} />"#, id, cx, cy, rx, ry, attr_str),

            SvgElementKind::Polygon { points, .. }
            | SvgElementKind::Polyline { points, .. } => {
                let tag = if matches!(&self.inner, SvgElementKind::Polygon { .. }) { "polygon" } else { "polyline" };
                let pts = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ");
                format!(r#"<{} points="{}"{} />"#, tag, pts, attr_str)
            }

            SvgElementKind::Path { id, d, .. } =>
                format!(r#"<path id="{}" d="{}"{} />"#, id, d, attr_str),

            SvgElementKind::Text { id, x, y, content, .. } =>
                format!(r#"<text id="{}" x="{}" y="{}"{}>{}</text>"#, id, x, y, attr_str, content),

            SvgElementKind::Group { id, elements, .. } => {
                let inner = elements.iter().map(|e| e.to_svg()).collect::<Vec<_>>().join("\n");
                format!("<g id=\"{}\"{} >\n{}\n</g>", id, attr_str, inner)
            }
        }
    }

    /// Read the style of this element (if any).
    pub fn style(&self) -> Option<&Style> { self.attr().style.as_ref() }

    /// Replaces the current style of this element (if any) with a new one.
    pub fn set_style(&mut self, style: Style) { self.attr_mut().style = Some(style); }

    /// Read the string ID of the masking element (if any).
    pub fn mask(&self) -> Option<&String> { self.attr().mask.as_ref() }

    /// Replaces the current masking element with a new one.
    pub fn set_mask(&mut self, style: Style) { self.attr_mut().style = Some(style); }

    /// Read the transformation string of the masking element (if any).
    pub fn transformation(&self) -> Option<&String> { self.attr().mask.as_ref() }

    /// Replaces the current transformation string for this element
    pub fn set_transformation(&mut self, style: Style) { self.attr_mut().style = Some(style); }

    pub(crate) fn attr_mut(&mut self) -> &mut SvgAttributes {
        match &mut self.inner {
            SvgElementKind::Line { attr, .. }
            | SvgElementKind::Rect { attr, .. }
            | SvgElementKind::Circle { attr, .. }
            | SvgElementKind::Ellipse { attr, .. }
            | SvgElementKind::Polygon { attr, .. }
            | SvgElementKind::Polyline { attr, .. }
            | SvgElementKind::Path { attr, .. }
            | SvgElementKind::Text { attr, .. }
            | SvgElementKind::Group { attr, .. } => attr,
        }
    }

    /// Returns a mutable reference to the elements in a group, if this element is a group.
    pub(crate) fn group_elements_mut(&mut self) -> Option<&mut Vec<SvgElement>> {
        match &mut self.inner {
            SvgElementKind::Group { elements, .. } => Some(elements),
            _ => None,
        }
    }

    fn attr(&self) -> &SvgAttributes {
        match &self.inner {
            SvgElementKind::Line { attr, .. }
            | SvgElementKind::Rect { attr, .. }
            | SvgElementKind::Circle { attr, .. }
            | SvgElementKind::Ellipse { attr, .. }
            | SvgElementKind::Polygon { attr, .. }
            | SvgElementKind::Polyline { attr, .. }
            | SvgElementKind::Path { attr, .. }
            | SvgElementKind::Text { attr, .. }
            | SvgElementKind::Group { attr, .. } => attr,
        }
    }
}

/// Roughly estimate the width in pixels of a text element.
///
/// The function assumes Latin text typeset in a generic sans-serif font.
///
/// # Arguments
/// - `text`: The text to measure. Assumes Latin characters; complex scripts are not modeled.
/// - `font_px`: The font size in CSS-equivalent pixels (e.g., `12.0` for 12px).
///
/// # Returns
/// Estimated width in pixels as an `f32`.
///
/// # Examples
/// ```
/// let w = estimate_text_width_heuristic("Crazy dog jumps...", 12.0);
/// # assert!(w > 0.0);
/// ```
pub fn estimate_text_width(text: &str, font_px: f32) -> f32 {
    // width factors in "em" (font size). Tuned for common sans-serif (Arial/Helvetica/DejaVu).
    const NARROW: f32 = 0.35; // i l ! | ' ` : ; . ,
    const NORMAL: f32 = 0.52; // default
    const WIDE:   f32 = 0.90; // M W @ # % & 0-9 often wider in many faces
    const SPACE:  f32 = 0.33; // space width roughly 1/3 em in many fonts

    let mut ems = 0.0f32;
    for ch in text.chars() {
        ems += match ch {
            ' ' => SPACE,
            // narrow-ish
            'i' | 'l' | 'I' | '!' | '|' | '\'' | '`' | ':' | ';' | '.' | ',' => NARROW,
            // wide-ish
            'M' | 'W' | '@' | '#' | '%' | '&' |
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => WIDE,
            // default bucket
            _ => NORMAL,
        };
    }
    ems * font_px
}
