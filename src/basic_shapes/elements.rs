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
    id: ElementID,
    attr: SvgAttributes,
    inner: SvgElementKind,
}


#[derive(Clone)]
enum SvgElementKind {
    /// Represents an SVG line element, defined by its start (x1, y1) and end (x2, y2) points.
    Line { x1: f32, y1: f32, x2: f32, y2: f32 },

    /// Represents an SVG rectangle element, with its position, width, and height.
    Rect { x: f32, y: f32, width: f32, height: f32 },

    /// Represents an SVG circle element, defined by its center (cx, cy) and radius `r`.
    Circle { cx: f32, cy: f32, r: f32 },

    /// Represents an SVG ellipse element, defined by its center (cx, cy) and radii (rx, ry).
    Ellipse { cx: f32, cy: f32, rx: f32, ry: f32 },

    /// Represents an SVG polygon element, defined by a list of 2D points.
    Polygon { points: Vec<(f32, f32)> },

    /// Represents an SVG polyline element, similar to a polygon but not closed.
    Polyline { points: Vec<(f32, f32)> },

    /// Represents an SVG path element, defined by a `d` attribute (path data string).
    Path { d: String },

    /// Represents an SVG text element, placed at (x, y) with the specified string content.
    Text { x: f32, y: f32, content: String },

    /// Represents an SVG group element, which can contain multiple child elements.
    Group { elements: Vec<SvgElement> },
}

// === Public API ===

impl SvgElement {

    pub fn line(id: impl Into<ElementID>, x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Line { x1, y1, x2, y2 } }
    }

    pub fn rect(id: impl Into<ElementID>, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Rect { x, y, width, height } }
    }

    pub fn circle(id: impl Into<ElementID>, cx: f32, cy: f32, r: f32) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Circle { cx, cy, r } }
    }

    pub fn ellipse(id: impl Into<ElementID>, cx: f32, cy: f32, rx: f32, ry: f32) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Ellipse { cx, cy, rx, ry } }
    }

    pub fn polygon(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Polygon { points } }
    }

    pub fn polyline(id: impl Into<ElementID>, points: Vec<(f32, f32)>) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Polyline { points } }
    }

    pub fn path(id: impl Into<ElementID>, d: impl Into<String>) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Path { d: d.into() } }
    }

    pub fn text(id: impl Into<ElementID>, x: f32, y: f32, content: impl Into<String>) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Text { x, y, content: content.into() } }
    }

    pub fn group(id: impl Into<ElementID>, elements: Vec<SvgElement>) -> Self {
        Self { id: id.into(), attr: SvgAttributes::default(), inner: SvgElementKind::Group { elements } }
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

    pub fn id(&self) -> &ElementID { &self.id }

    /// Converts the element into an SVG XML fragment
    pub fn to_svg(&self) -> String {
        let attr_str = self.attr.to_svg_fragment();
        let id = &self.id;

        match &self.inner {
            SvgElementKind::Line { x1, y1, x2, y2 } =>
                format!(r#"<line id="{}" x1="{}" y1="{}" x2="{}" y2="{}"{} />"#, id, x1, y1, x2, y2, attr_str),

            SvgElementKind::Rect { x, y, width, height } =>
                format!(r#"<rect id="{}" x="{}" y="{}" width="{}" height="{}"{} />"#, id, x, y, width, height, attr_str),

            SvgElementKind::Circle { cx, cy, r } =>
                format!(r#"<circle id="{}" cx="{}" cy="{}" r="{}"{} />"#, id, cx, cy, r, attr_str),

            SvgElementKind::Ellipse { cx, cy, rx, ry } =>
                format!(r#"<ellipse id="{}" cx="{}" cy="{}" rx="{}" ry="{}"{} />"#, id, cx, cy, rx, ry, attr_str),

            SvgElementKind::Polygon { points }
            | SvgElementKind::Polyline { points } => {
                let tag = if matches!(&self.inner, SvgElementKind::Polygon { .. }) { "polygon" } else { "polyline" };
                let pts = points.iter().map(|(x, y)| format!("{},{}", x, y)).collect::<Vec<_>>().join(" ");
                format!(r#"<{} id="{}" points="{}"{} />"#, tag, id, pts, attr_str)
            }

            SvgElementKind::Path { d } =>
                format!(r#"<path id="{}" d="{}"{} />"#, id, d, attr_str),

            SvgElementKind::Text { x, y, content } =>
                format!(r#"<text id="{}" x="{}" y="{}"{}>{}</text>"#, id, x, y, attr_str, content),

            SvgElementKind::Group { elements } => {
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
    pub fn set_mask(&mut self, style: String) { self.attr_mut().mask = Some(style); }

    /// Read the transformation string of the masking element (if any).
    pub fn transformation(&self) -> Option<&String> { self.attr().transform.as_ref() }

    /// Replaces the current transformation string for this element
    pub fn set_transformation(&mut self, style: String) { self.attr_mut().transform = Some(style); }

    /// Replaces the current transformation defined for this element with a translation by (dx, dy)
    pub fn translate(&mut self, dx: f32, dy: f32)  { self.attr_mut().transform = Some(format!("translate({dx},{dy})"))}

    pub(crate) fn attr_mut(&mut self) -> &mut SvgAttributes { &mut self.attr }

    /// Returns a mutable reference to the elements in a group, if this element is a group.
    #[allow(unused)] // This is necessary for PyO3 binding!
    pub(crate) fn group_elements_mut(&mut self) -> Option<&mut Vec<SvgElement>> {
        match &mut self.inner {
            SvgElementKind::Group { elements, .. } => Some(elements),
            _ => None,
        }
    }

    /// Recursive attempt to add an element to a group given its ID
    ///
    /// Returns true if an element has been actually added; false when failed.
    /// This method is necessary to have Python API working
    pub(crate) fn add_to_group(&mut self, target_id: &ElementID, element_to_add: SvgElement) -> bool {

        let is_target = &self.id == target_id;

        match &mut self.inner {
            SvgElementKind::Group { elements } if is_target => {
                elements.push(element_to_add);
                true
            }
            SvgElementKind::Group { elements } => {
                for child in elements.iter_mut() {
                    if child.add_to_group(target_id, element_to_add.clone()) {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn attr(&self) -> &SvgAttributes { &self.attr }
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
/// # use visualife::basic_shapes::estimate_text_width;
/// let w = estimate_text_width("Crazy dog jumps...", 12.0);
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
