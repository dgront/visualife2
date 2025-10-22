use crate::{ElementID};
use crate::basic_shapes::SvgElement;
use crate::styling::Style;

/// Represents a labeled node of a mind map.
///
/// Most often you create a node using the [`grow_node()`](crate::mindmap::Mindmap::grow_node)
/// or [`place_node()`](crate::mindmap::Mindmap::place_node) methods of the [`Mindmap`](crate::mindmap::Mindmap) struct.
///
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier for the node.
    pub id: ElementID,
    /// Text label displayed inside the node.
    pub label: String,
    /// X-coordinate of the node center.
    pub cx: f32,
    /// Y-coordinate of the node center.
    pub cy: f32,
    /// Radius of the circular node.
    pub radius: f32,
    circle_style: Option<Style>,
    text_style: Option<Style>,
}

impl Node {
    /// Creates a new `Node` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the node (can be a string or `ElementID`).
    /// * `label` - Text to display inside the node.
    /// * `cx` - X-coordinate of the node center.
    /// * `cy` - Y-coordinate of the node center.
    /// * `radius` - Radius of the circular node.
    ///
    /// # Example
    /// ```
    /// use visualife::mindmap::Node;
    /// let node = Node::new("n1", "A", 100.0, 100.0, 10.0);
    /// let svg_elem = node.create_element();
    /// let svg_str = svg_elem.to_svg();
    /// # assert_eq!(svg_str, "<g id=\"n1\" >\n<circle id=\"c:n1\" cx=\"100\" cy=\"100\" r=\"10\" />\n<text id=\"t:n1\" x=\"100\" y=\"100\" style=\"text-anchor:middle;dominant-baseline:middle;\">A</text>\n</g>");
    /// # assert_eq!(node.id, "n1".into());
    /// # assert_eq!(node.cx, 100.0);
    /// ```
    pub fn new(id: impl Into<ElementID>, label: &str, cx: f32, cy: f32, radius: f32) -> Self {
        return Node { id: id.into(), label: label.to_string(), cx, cy, radius, circle_style: None, text_style: None }
    }

    /// Creates an SVG group element containing a styled circle and text label.
    ///
    /// `id` of the node is used as the ID for the whole group. The circle and text use prefixed IDs
    /// (`c:<id>` and `t:<id>` respectively) and inherit styles if provided.
    ///
    /// # Example
    /// ```
    /// use visualife::mindmap::Node;
    /// let node = Node::new("n1", "A", 100.0, 100.0, 10.0);
    /// let element = node.create_element();
    /// let svg = element.to_svg();
    /// ```
    pub fn create_element(&self) -> SvgElement {

        let mut circle = SvgElement::circle(self.id.new_with_prefix("c:"), self.cx, self.cy, self.radius);
        if let Some(style) = &self.circle_style {
            circle = circle.with_style(style.clone());
        }
        let mut text = SvgElement::text(self.id.new_with_prefix("t:"), self.cx, self.cy, self.label.as_str());
        if let Some(style) = &self.text_style {
            if style.text_anchor.is_none() {
                text = text.with_style(style.clone().dominant_baseline("middle").text_anchor("middle"));
            } else {
                text = text.with_style(style.clone());
            }
        } else {
            text = text.with_style(Style::new().dominant_baseline("middle").text_anchor("middle"));
        }
        return SvgElement::group(self.id.clone(), vec![circle, text]);
    }

    /// Applies a style to the circle element of the node.
    ///
    /// # Example
    /// ```
    /// use visualife::mindmap::Node;
    /// use visualife::styling::Style;
    /// let mut node = Node::new("n1", "A", 100.0, 100.0, 10.0);
    /// node.with_style(Style::new().fill("red").stroke("black"));
    /// ```
    pub fn with_style(&mut self, style: Style) -> &mut Self {
        self.circle_style = Some(style);
        self
    }

    /// Applies a style to the text element of the node.
    ///
    /// # Arguments
    ///
    /// * `style` - A `Style` instance to apply to the text.
    ///
    /// # Example
    /// ```
    /// use visualife::mindmap::Node;
    /// use visualife::styling::Style;
    /// let mut node = Node::new("n1", "A", 100.0, 100.0, 10.0);
    /// node.with_text_style(Style::new().fill("black").font_size("12"));
    /// ```
    pub fn with_text_style(&mut self, style: Style) -> &mut Self {
        self.text_style = Some(style);
        self
    }
}

#[cfg(test)]
mod test_node {
    use crate::mindmap::node::Node;
    use crate::normalize_whitespace;
    use crate::styling::{Style};

    #[test]
    fn node_to_svg() {
        let (x1, y1, r1)  = (100.0_f32, 100.0_f32, 10.0_f32);
        let mut nodeA = Node::new("n1", "A", x1, y1, r1);
        nodeA.with_style(Style::new().fill("red").stroke("black"));
        let expected = r#"<g id="n1" >
    <circle id="c:n1" cx="100" cy="100" r="10" style="fill:red;stroke:black;" />
    <text id="t:n1" x="100" y="100" style="text-anchor:middle;dominant-baseline:middle;">A</text>
</g>"#;
        assert_eq!(normalize_whitespace(&nodeA.create_element().to_svg()), normalize_whitespace(&expected));
    }
}