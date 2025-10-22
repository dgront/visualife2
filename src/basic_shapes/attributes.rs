use crate::styling::Style;

/// Shared attribute structure for all SVG elements.
///
/// Every [`SvgElement`](crate::basic_shapes::SvgElement) can provide its transformation (stored as a string),
/// id string of the masking element as well as its [`Style`].
#[derive(Clone, Default)]
pub struct SvgAttributes {
    pub style: Option<Style>,
    pub transform: Option<String>,
    pub mask: Option<String>,
}

impl SvgAttributes {
    pub fn to_svg_fragment(&self) -> String {
        let mut out = String::new();
        if let Some(s) = &self.style {
            out.push_str(&format!(" {}", s.to_svg()));
        }
        if let Some(t) = &self.transform {
            out.push_str(&format!(r#" transform="{}""#, t));
        }
        if let Some(m) = &self.mask {
            out.push_str(&format!(r#" mask="url(#{})""#, m));
        }
        out
    }
}
